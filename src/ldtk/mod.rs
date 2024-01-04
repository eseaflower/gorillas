use std::{io::BufReader, path::Path};

mod ldtk_schema;

pub use ldtk_schema::LdtkJson as Project;

pub fn load(path: impl AsRef<Path>) -> Result<Project, std::io::Error> {
    let reader = BufReader::new(std::fs::File::open(path)?);
    Ok(serde_json::from_reader::<_, Project>(reader)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ldtk() {
        let file = "assets/test.ldtk";
        let project = load(file).expect("failed to load ldtk file");

        project.levels.iter().for_each(|level| {
            println!("level: {}", level.identifier);
            level.layer_instances.iter().for_each(|layer| {
                layer.iter().for_each(|instance| {
                    println!("instance: {}", instance.identifier);

                    instance.grid_tiles.iter().for_each(|tile| {
                        println!("tile: {:?}", tile);
                    });
                });
            });
        });
    }
}
