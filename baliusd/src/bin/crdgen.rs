use kube::CustomResourceExt;

fn main() {
    print!(
        "{}",
        serde_yaml::to_string(&baliusd::k8s::BaliusWorker::crd()).unwrap()
    )
}
