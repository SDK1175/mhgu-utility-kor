"""Exact native skill/jewel facts for the read-only set builder.

No rarity-to-progression guesses. Recipe.gate preserves the opaque native
creation gate for the runtime checker; it is NOT an HR value. Material stock
must never determine progression availability. All IDs have named namespaces.
"""
import json
import os
import sqlite3
import struct
import sys
from pathlib import Path
from game_names import strings
try:  # Korean fork: the translated explanations
    sys.path.insert(0, str(Path(__file__).resolve().parent / 'kor'))
    from skill_effects_ko import descriptions, pack
except ImportError:
    from skill_effects import descriptions, pack
from paths import DATA, GENERATED, database

THRESHOLDS = (-20, -15, -10, 10, 15, 20, 25)
TORSO_UP, SKILL_PLUS, TALISMAN_BOOST = 203, 204, 205


def records(path, size, magic):
    raw = path.read_bytes()
    if len(raw) < 8 or raw[:4] != magic:
        raise ValueError(f'Unsupported native table: {path.name}')
    count = struct.unpack_from('<I', raw, 4)[0]
    if len(raw) != 8 + count * size:
        raise ValueError(f'Native table record mismatch: {path.name}')
    return [raw[o:o + size] for o in range(8, len(raw), size)]


def activations(rows, names):
    """Repeated IDs encode saturation; retain the first crossing of zero."""
    result = []
    for tree, row in enumerate(rows):
        if struct.unpack_from('<I', row)[0] != tree:
            raise ValueError('Non-indexed native skill tree')
        values = struct.unpack_from('<7H', row, 5)
        for skill in sorted(set(values) - {0}):
            thresholds = [t for t, value in zip(THRESHOLDS, values) if value == skill]
            if min(thresholds) < 0 < max(thresholds):
                raise ValueError('Activation spans both signs')
            threshold = min(thresholds, key=abs)
            if skill >= len(names):
                raise ValueError('Activation name missing')
            result.append((tree, threshold, names[skill]))
    return sorted(result, key=lambda r: (r[0], r[1]))


def facts(db, catalog, game_dir):
    root = game_dir / 'romfs/nativeNX'
    trees = strings(root / 'eng/table/skillTypeData_eng.gmd')[::2]
    names = strings(root / 'eng/table/skillData_eng.gmd')[::2]
    tree_rows = records(root / 'table/skillTypeData.skt', 19, bytes.fromhex('00000842'))
    if len(trees) != len(tree_rows) or len(trees) > 256:
        raise ValueError('Native skill tree names mismatch')
    if [trees[i] for i in (TORSO_UP, SKILL_PLUS, TALISMAN_BOOST)] != ['Torso Up', 'Secret Arts', 'Talisman Boost']:
        raise ValueError('Native special skill identities changed')
    native_tree = {name: i for i, name in enumerate(trees)}
    armor_names = sorted({r[0] for r in db.execute('select t.name from item_to_skill_tree s join skill_trees t on t._id=s.skill_tree_id join armor a on a._id=s.item_id')})
    armor_trees = [native_tree[n] for n in armor_names]
    entry_ids = {r['name']: i for i, r in enumerate(sorted(catalog['items'].values(), key=lambda r: r['name']))}
    item_names = strings(root / 'eng/table/itemData_eng.gmd')[::2]
    deco_rows = records(root / 'table/decoData.deco', 5, bytes.fromhex('6666aa41'))
    recipe_rows = records(root / 'table/decoCreate.dcd', 16, bytes.fromhex('0000803f'))
    db_decos = list(db.execute('select i._id,i.name,d.num_slots from decorations d join items i using(_id) order by i._id'))
    # Native recipes identify the first jewel item; every following native deco
    # record has its item at this base + index. Cross-check EVERY real entry.
    base = min(struct.unpack_from('<H', r)[0] for r in recipe_rows)
    expected = {key: (name, slots) for key, name, slots in db_decos}
    points, materials, recipes, jewels = [], [], [], []
    for offset, row in enumerate(deco_rows):
        native = base + offset
        name = item_names[native]
        if name == 'DUMMY':
            if any(row[1:]) or native in expected:
                raise ValueError('Nonempty dummy jewel')
            continue
        if expected.pop(native, None) != (name, row[0]) or name not in entry_ids:
            raise ValueError(f'Unmatched native jewel: {native} {name}')
        skill_offset = len(points)
        for tree, value in ((row[1], row[2]), (row[3], row[4])):
            value = value if value < 128 else value - 256
            if tree:
                if tree >= len(trees) or not value:
                    raise ValueError('Invalid jewel skill')
                points.append((tree, value))
            elif value:
                raise ValueError('Jewel points without a tree')
        db_points = sorted((native_tree[n], v) for n, v in db.execute('select t.name,s.point_value from item_to_skill_tree s join skill_trees t on t._id=s.skill_tree_id where s.item_id=?', (native,)))
        if db_points != sorted(points[skill_offset:]):
            raise ValueError(f'Native/database skill mismatch: {name}')
        recipe_offset = len(recipes)
        for recipe in recipe_rows:
            item, gate = struct.unpack_from('<HH', recipe)
            if item != native:
                continue
            material_offset = len(materials)
            for start in range(4, 16, 3):
                item_id, quantity = struct.unpack_from('<HB', recipe, start)
                if item_id:
                    material_name = item_names[item_id]
                    if material_name not in entry_ids or not quantity:
                        raise ValueError(f'Unmapped jewel material: {material_name}')
                    materials.append((entry_ids[material_name], quantity))
                elif quantity:
                    raise ValueError('Material quantity without item')
            recipes.append((gate, material_offset, len(materials) - material_offset))
        if len(recipes) == recipe_offset:
            raise ValueError(f'Jewel without native recipe: {name}')
        jewels.append((native, entry_ids[name], row[0], skill_offset, len(points) - skill_offset, recipe_offset, len(recipes) - recipe_offset))
    if expected:
        raise ValueError('Database jewels absent from native table')
    return trees, armor_trees, activations(tree_rows, names), jewels, points, recipes, materials


def armor_recipes(catalog, game_dir, materials):
    """Native smithy alternatives, retaining slot-local recipe/variant identity.

    Four physical inputs precede a separate point category. The final three
    item/quantity pairs are output scraps and MUST NOT become requirements.
    Unnamed native DUMMY inputs remain unknown (65535), never silently omitted.
    Armor materials contain catalog IDs only: quantities do not affect progression.
    """
    root = game_dir / 'romfs/nativeNX'
    item_names = strings(root / 'eng/table/itemData_eng.gmd')[::2]
    entry_ids = {r['name']: i for i, r in enumerate(sorted(catalog['items'].values(), key=lambda r: r['name']))}
    result = []
    # Shared slices save repeated ingredient lists without losing alternatives.
    slices = {}
    for slot in range(5):
        rows = records(root / f'table/armorCreateA{slot:02}.acrd', 42, bytes.fromhex('0000803f'))
        for index, row in enumerate(rows):
            native = struct.unpack_from('<4H', row)
            gate = struct.unpack_from('<H', row, 8)[0]
            inputs = []
            for offset in range(10, 26, 4):
                item, quantity, kind = struct.unpack_from('<HBB', row, offset)
                if kind not in (0, 1, 2) or (not item and quantity):
                    raise ValueError('Unsupported armor ingredient encoding')
                if item:
                    if item >= len(item_names) or not quantity:
                        raise ValueError('Invalid native armor ingredient')
                    name = item_names[item]
                    if name not in entry_ids and name != 'DUMMY':
                        raise ValueError(f'Unmapped armor material: {name}')
                    inputs.append(entry_ids.get(name, 65535))
            key = tuple(sorted(set(inputs)))
            if key not in slices:
                slices[key] = len(materials)
                materials.extend(key)
            category, count = struct.unpack_from('<HH', row, 26)
            if bool(category) != bool(count):
                raise ValueError('Incomplete armor point requirement')
            result.append((native, gate, index, slices[key], len(key), slot, category, count))
    if len(materials) >= 65536:
        raise ValueError('Set materials exceed u16 offsets')
    return result


def emit(db, catalog, game_dir):
    trees, armor_trees, active, jewels, points, recipes, materials = facts(db, catalog, game_dir)
    armor_materials = []
    armor = armor_recipes(catalog, game_dir, armor_materials)
    q = lambda s: json.dumps(s, ensure_ascii=False)
    out = ['// Generated from owned native MHGU tables; exact DB cross-checks, no progression guesses.',
           'pub const TORSO_UP:u8=203;pub const SKILL_PLUS:u8=204;pub const TALISMAN_BOOST:u8=205;',
           "pub struct Activation{pub tree:u8,pub points:i8,pub name:&'static str}",
           'pub struct Decoration{pub native:u16,pub entry:u16,pub slots:u8,pub skill:u16,pub skill_count:u8,pub recipe:u16,pub recipe_count:u8}',
           '// gate and point_category are opaque native conditions, NOT hunter rank/item IDs.',
           'pub struct ArmorRecipe{pub native:[u16;4],pub gate:u16,pub index:u16,pub material:u16,pub material_count:u8,pub slot:u8,pub point_category:u16,pub point_count:u16}',
           'pub struct Recipe{pub gate:u16,pub material:u16,pub material_count:u8}',
           'pub static TREE_NAMES:&[&str]=&[' + ','.join(map(q, trees)) + '];',
           'pub static ARMOR_TREES:&[u8]=&[' + ','.join(map(str, armor_trees)) + '];',
           'pub static ACTIVATIONS:&[Activation]=&[']
    out.extend(f'Activation{{tree:{t},points:{p},name:{q(n)}}},' for t, p, n in active)
    out += ['];', 'pub static DECO:&[Decoration]=&[']
    out.extend('Decoration{' + ','.join(f'{k}:{v}' for k, v in zip(('native','entry','slots','skill','skill_count','recipe','recipe_count'), row)) + '},' for row in jewels)
    out += ['];', 'pub static POINTS:&[(u8,i8)]=&[' + ','.join(str(p).replace(' ', '') for p in points) + '];', 'pub static RECIPES:&[Recipe]=&[']
    out.extend(f'Recipe{{gate:{g},material:{m},material_count:{c}}},' for g,m,c in recipes)
    out += ['];', 'pub static MATERIALS:&[(u16,u8)]=&[' + ','.join(str(p).replace(' ', '') for p in materials) + '];']
    out += ['// Physical input reachability only; output scraps and stock quantities excluded.',
            'pub static ARMOR_MATERIALS:&[u16]=&[' + ','.join(map(str,armor_materials)) + '];',
            'pub static ARMOR_RECIPES:&[ArmorRecipe]=&[']
    for native,g,index,m,count,slot,category,quantity in armor:
        out.append(f'ArmorRecipe{{native:{list(native)},gate:{g},index:{index},material:{m},material_count:{count},slot:{slot},point_category:{category},point_count:{quantity}}},')
    rows = strings(game_dir / 'romfs/nativeNX/eng/table/skillData_eng.gmd')
    native = {rows[i]: ' '.join(rows[i+1].split()) for i in range(0,len(rows),2) if rows[i] and rows[i+1].strip()}
    effects = descriptions(native)
    blob,pairs,offsets = pack([effects[name][1] for _,_,name in active])
    out += ['];', '// Reviewed full effects, byte-pair packed to preserve the shared plugin budget.',
            'pub static EFFECTS:&[(u32,u16)]=&['+','.join(str(p).replace(' ','')for p in offsets)+'];',
            'pub static EFFECT_PAIRS:&[(u8,u8);256]=&['+','.join(str(p).replace(' ','')for p in pairs)+'];',
            'pub static EFFECT_TEXT:&[u8]=&['+','.join(map(str,blob))+'];']
    return '\n'.join(out) + '\n'


if __name__ == '__main__':
    result = emit(database(), json.loads((DATA / 'catalog.json').read_text()), Path(os.environ['MHGU_GAME_DIR']))
    (GENERATED / 'set_data.rs').write_text(result)
    print('Generated exact set-builder skill and decoration tables')
