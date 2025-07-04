use kube_client::config::Kubeconfig;

pub fn get_context() -> Option<String> {
    Kubeconfig::read().ok()?.current_context
}
