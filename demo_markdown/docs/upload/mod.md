# Upload

```rust demo
use send_wrapper::SendWrapper;

// let message = use_message();
let custom_request = move |file_list: SendWrapper<FileList>| {
    // message.create(
    //     format!("Number of uploaded files: {}", file_list.length()),
    //     MessageVariant::Success,
    //     Default::default(),
    // );
};

view!{
    <Upload>
        <Button>
            "upload"
        </Button>
    </Upload>
}
```

### Drag to upload

```rust demo

// let message = use_message();
let custom_request = move |file_list: FileList| {
    // message.create(
    //     format!("Number of uploaded files: {}", file_list.length()),
    //     MessageVariant::Success,
    //     Default::default(),
    // );
};

view! {
    <Upload custom_request>
        <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
    </Upload>
}
```

### Upload Props

| Name           | Type                             | Default              | Description                          |
| -------------- | -------------------------------- | -------------------- | ------------------------------------ |
| accept         | `MaybeSignal<String>`            | `Default::default()` | The accept type of upload.           |
| multiple       | `MaybeSignal<bool>`              | `false`              | Allow multiple files to be selected. |
| custom_request | `Option<Callback<FileList, ()>>` | `Default::default()` | Customize upload request.            |
| children       | `Children`                       |                      | Upload's content.                    |

### UploadDragger Props

| Name     | Type       | Default | Description              |
| -------- | ---------- | ------- | ------------------------ |
| children | `Children` |         | UploadDragger's content. |
