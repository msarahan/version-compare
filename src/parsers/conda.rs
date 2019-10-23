use crate::version_part::VersionPart;
use crate::custom_parts::pep440::PEP440String;

/// Split the given version string, in it's version parts.
pub fn conda_parser(
    version: &str,
) -> Option<Vec<VersionPart>> {
    // version len may be a bit wasteful of memory.  Let's start there and tune as necessary.
    let mut parts = Vec::with_capacity(version.len()/2);

    // Split at epoch
    let epoch_split: Vec<&str> = version.split("!").collect();
    let post_epoch_split: &str = match epoch_split.len() {
        2 => {
            parts.push(VersionPart::Epoch(epoch_split[0].parse().unwrap()));
            epoch_split[1]
        },
        1 => {
            epoch_split[0]
        },
        _ => panic!("Duplicated epoch separator (!)")
    };

    // Get any local version string
    let local_version_split: Vec<&str> = post_epoch_split.split("+").collect();
    let local: &str = match local_version_split.len() {
        1 => "",
        2 => local_version_split[1],
        _ => panic!("duplicated local version separator (+)")
    };

    // Split at periods
    let mut version_split: Vec<&str> = local_version_split[0].split(|c| c == '_' || c == '.')
        .collect();
    let local_split: Vec<&str> = local.split(|c| c == '_' || c == '.').collect();
    version_split.extend(local_split);

    // Loop over the parts, and parse them
    for part in version_split {
        println!("{}", part);
        // Skip empty parts
        if part.is_empty() {
            continue;
        }

        // Try to parse the value as an number
        match part.parse::<i32>() {
            Ok(number) => {
                // Push the number part to the vector, and set the has number flag
                parts.push(VersionPart::Integer(number));
            }
            Err(_) => {
                // Push the text part to the vector
                parts.push(VersionPart::PEP440String(PEP440String::from(part)));
            }
        }
    }

    if parts.is_empty() && version.is_empty() {
        parts.push(VersionPart::Empty);
    }

    // Return the list of parts
    Some(parts)
}

#[cfg(test)]
mod tests {
    use super::conda_parser;

    # [test]
    fn test_less_specific_less_than_more_specific() {
        // 0.4 < 0.4.0
        let parts = conda_parser("0.4").unwrap();
        assert_eq!(parts.len(), 2);
    }
}