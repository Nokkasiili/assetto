pub type SysResult<T = ()> = anyhow::Result<T>;

type SystemFn<Input> = Box<dyn FnMut(&mut Input) -> SysResult>;

struct SystemExecutor<Input> {
    systems: Vec<System<Input>>,
}
struct System<Input> {
    function: SystemFn<Input>,
}

impl<Input> System<Input> {
    fn from_fn<F: FnMut(&mut Input) -> SysResult + 'static>(f: F) -> Self {
        Self {
            function: Box::new(f),
        }
    }
}

impl<Input> SystemExecutor<Input> {
    pub fn add_system(
        &mut self,
        system: impl FnMut(&mut Input) -> SysResult + 'static,
    ) -> &mut Self {
        let system = System::from_fn(system);
        self.systems.push(system);
        self
    }

    pub fn run(&mut self, input: &mut Input) {
        for (i, system) in self.systems.iter_mut().enumerate() {
            let result = (system.function)(input);
            if let Err(e) = result {
                log::error!("System {} returned an error; this is a bug", e);
            }
        }
    }
}
