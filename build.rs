fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(protobuf_codegen_pure::Customize {
            ..Default::default()
        })
        .out_dir("src/ms_coco/protos")
        .input("src/ms_coco/protos/labelmap.proto")
        .include("src/ms_coco/protos")
        .run()
        .expect("protoc");
}
