use crate::version_part::VersionPart;
use core::panicking::panic_fmt;

/// Split the given version string, in it's version parts.
pub fn conda_parser(
    version: &str,
) -> Option<Vec<VersionPart>> {
    // version len may be a bit wasteful of memory.  Let's start there and tune as necessary.
    let mut parts = Vec::with_capacity(version.len()/2);

    // Split at epoch
    let epoch_split: Vec<&str> = version.split("!").collect();
    match epoch_split.len() {
        2 => parts.push(VersionPart::Epoch(epoch_split[0].parse().unwrap())),
        1 => {},
        _ => panic!("Duplicated epoch separator (!)")
    };



    // Get any local version string
    let local_version_split: Vec<&str> = epoch_split[-1].split("+").collect();
    let local: Vec<&str> = match local_version_split.len() {
        1 => Vec::new(),
        2 => local_version_split[-1].replace("_", ".").split(".").collect(),
        _ => panic!("duplicated local version separator (+)")
    };

    // Split at periods
    let mut version_split: Vec<&str> = local_version_split[0].replace("_", ".").split(".");
    version_split.extend(&local);

//    # split components into runs of numerals and non-numerals,
//    # convert numerals to int, handle special strings
//    for v in (self.version, self.local):
//    for k in range(len(v)):
//        c = version_split_re.findall(v[k])
//    if not c:
//        raise InvalidVersionSpec(vstr, "empty version component")
//    for j in range(len(c)):
//    if c[j].isdigit():
//        c[j] = int(c[j])
//    elif c[j] == 'post':
//    # ensure number < 'post' == infinity
//    c[j] = float('inf')
//    elif c[j] == 'dev':
//    # ensure '*' < 'DEV' < '_' < 'a' < number
//    # by upper-casing (all other strings are lower case)
//    c[j] = 'DEV'
//    if v[k][0].isdigit():
//        v[k] = c
//    else:
//    # components shall start with a number to keep numbers and
//    # strings in phase => prepend fillvalue
//    v[k] = [self.fillvalue] + c

    // Loop over the parts, and parse them
    for part in split {
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
                parts.push(VersionPart::PEP440String(part));
            }
        }
    }

    if parts.is_empty() && version.is_empty() {
        parts.push(VersionPart::Empty);
    }

    // Return the list of parts
    Some(parts)
}