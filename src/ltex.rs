use std::collections::{HashMap, HashSet};

use tes3::esp::{Landscape, LandscapeTexture, Plugin, TES3Object};

fn build_replacement_table(
    replacements: HashMap<String, String>,
    textures: &[LandscapeTexture],
) -> Result<HashMap<u32, u32>, String> {
    let mut replace = HashMap::new();
    for (current, replacement) in replacements {
        if let Some(current_ltex) = textures
            .iter()
            .find(|ltex| ltex.id.eq_ignore_ascii_case(&current))
        {
            if let Some(replacement_ltex) = textures
                .iter()
                .find(|ltex| ltex.id.eq_ignore_ascii_case(&replacement))
            {
                if replace
                    .insert(current_ltex.index + 1, replacement_ltex.index + 1)
                    .is_some()
                {
                    return Err(format!("Found multiple replacements for LTEX {}", current));
                }
            } else {
                println!(
                    "Not replacing LTEX {} with {} because the latter doesn't exist",
                    current, replacement
                );
            }
        } else {
            println!("Not replacing LTEX {} because it doesn't exist", current);
        }
    }
    Ok(replace)
}

fn replace_textures(plugin: &mut Plugin, replace: HashMap<u32, u32>) -> HashSet<u32> {
    let mut used = HashSet::new();
    for landscape in plugin.objects_of_type_mut::<Landscape>() {
        for row in landscape.texture_indices.data.as_mut_slice() {
            for index in row {
                if let Some(replacement) = replace.get(&(*index as u32)) {
                    *index = *replacement as u16;
                }
                if *index > 0 {
                    used.insert((*index as u32) - 1);
                }
            }
        }
    }
    used
}

fn is_ltex(object: &TES3Object) -> bool {
    matches!(object, TES3Object::LandscapeTexture(_))
}

pub fn deduplicate_ltex(
    plugin: &mut Plugin,
    replacements: HashMap<String, String>,
) -> Result<(), String> {
    let mut textures: Vec<LandscapeTexture> = plugin
        .objects_of_type::<LandscapeTexture>()
        .cloned()
        .collect();
    if textures.is_empty() {
        return Err("Plugin did not contain any landscape textures".to_string());
    }
    let replace = build_replacement_table(replacements, &textures)?;
    let used = replace_textures(plugin, replace);

    let original_size = textures.len();
    textures.retain(|ltex| used.contains(&ltex.index));
    let removed = original_size - textures.len();
    println!("Found {} unused landscape textures", removed);

    let mut new_indices = HashMap::new();
    for (index, texture) in textures.iter_mut().enumerate() {
        let current = texture.index;
        texture.index = index as u32;
        new_indices.insert(current + 1, texture.index + 1);
    }
    replace_textures(plugin, new_indices);

    let (first_ltex, _) = plugin
        .objects
        .iter()
        .enumerate()
        .find(|(_, object)| is_ltex(object))
        .unwrap();
    plugin.objects.retain(|object| !is_ltex(object));
    plugin.objects.splice(
        first_ltex..first_ltex,
        textures.into_iter().map(TES3Object::LandscapeTexture),
    );

    Ok(())
}
