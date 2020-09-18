use smol::Executor;

#[smol_potat::main]
async fn main() {
    let ex = Executor::new();

    ex.run(async {
        println!("Hello, world!");
    })
    .await;
}
