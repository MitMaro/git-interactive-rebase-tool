pub(crate) trait UpdateHandlerFn: Fn() + Sync + Send {}

impl<FN: Fn() + Sync + Send> UpdateHandlerFn for FN {}
