use anyhow::Result;
use easeprobe::start;

#[tokio::main]
async fn main() -> Result<()> {
    start()?;
    // manager::set_channel("test").await;

    // let prober = HttpProber::new(
    //     "HttpProbe",
    //     "http://www.google.com",
    //     Method::GET,
    //     HashMap::new(),
    //     None,
    //     Duration::from_secs(5),
    //     Duration::from_secs(5),
    // );

    // let mut probers: Vec<Arc<RwLock<dyn Prober>>> = vec![Arc::new(RwLock::new(prober))];
    // config_probers(&mut probers).await;

    // let mut notifier = DefaultNotifier::default();
    // notifier.send_func = Some(|_: &str, _: &str| -> Result<()> { Ok(()) });

    // let mut notifiers: Vec<Arc<RwLock<dyn Notifier>>> = vec![Arc::new(RwLock::new(notifier))];
    // config_notifiers(&mut notifiers).await;

    // manager::set_notifiers(notifiers.clone()).await;
    // manager::set_probers(probers.clone()).await;

    // run_probers(probers);

    // tokio::time::sleep(Duration::new(30, 1000)).await;
    Ok(())
}
