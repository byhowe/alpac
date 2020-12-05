use alpac::Ingredients;

pub fn print_version(ingredients: &Ingredients)
{
  let mut versions = ingredients.available_versions();
  if versions.len() == 0 {
    println!("This recipe does not define any versions.");
  } else {
    println!("Available versions are:");
    versions.sort();
    versions.reverse();
    for version in versions {
      println!("  - {}", version);
    }
  }
}
