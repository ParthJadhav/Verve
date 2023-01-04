use rust_search::SearchBuilder;
use std::path::Path;
use strsim::jaro_winkler;

fn file_name_from_path(path: &str) -> String {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    return file_name.to_string();
}

pub fn similarity_sort(vector: &mut Vec<String>, input: &str) {
    vector.sort_by(|a, b| {
        let input = input.to_lowercase();
        let a = file_name_from_path(a).to_lowercase();
        let b = file_name_from_path(b).to_lowercase();
        let a = jaro_winkler(a.as_str(), input.as_str());
        let b = jaro_winkler(b.as_str(), input.as_str());
        b.partial_cmp(&a).unwrap()
    });
}

pub fn search(
    input: &str,
    search_locations: Vec<&str>,
    extension: Option<&str>,
    depth: Option<usize>,
) -> Vec<String> {
    let (location, more_locations) = search_locations.split_first().unwrap();
    let mut result: Vec<String> = SearchBuilder::default()
        .search_input(input)
        .location(location)
        .more_locations(more_locations.to_vec())
        .depth(depth.unwrap_or(1))
        .ext(extension.unwrap_or("*"))
        .limit(5)
        .ignore_case()
        .build()
        .collect();
    similarity_sort(&mut result, input);
    return result;
}
