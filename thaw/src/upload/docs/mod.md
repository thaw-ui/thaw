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
let toaster = ToasterInjection::expect_context();

let custom_request = move |file_list: FileList| {
    let len = file_list.length();
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastBody>
                {format!("Number of uploaded files: {len}")}
            </ToastBody>
        </Toast>
    }, Default::default());
};

view! {
    <Upload custom_request>
        <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
    </Upload>
}
```

### Upload Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| accept | `Signal<String>` | `Default::default()` | The accept type of upload. |
| multiple | `Signal<bool>` | `false` | Allow multiple files to be selected. |
| custom_request | `Option<ArcOneCallback<FileList>>` | `None` | Customize upload request. |
| children | `Children` |  |  |

### UploadDragger Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
