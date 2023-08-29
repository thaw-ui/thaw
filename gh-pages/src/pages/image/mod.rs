use leptos::*;
use melt_ui::*;

#[component]
pub fn ImagePage() -> impl IntoView {
    view! {
        <>
            <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
            <Image width="200px" height="200px"/>
        </>
    }
}
