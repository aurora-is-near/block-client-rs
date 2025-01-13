fn main() -> std::io::Result<()> {
    let proto_files = walkdir::WalkDir::new("proto")
        .into_iter()
        .filter_map(|maybe_entry| match maybe_entry {
            Ok(entry) if is_proto_file(&entry) => Some(entry.path().display().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();

    tonic_build::configure().build_client(true).compile_protos(&proto_files, &["proto"])
}

fn is_proto_file(entry: &walkdir::DirEntry) -> bool {
    entry.path().extension().is_some_and(|ext| ext == "proto")
}
