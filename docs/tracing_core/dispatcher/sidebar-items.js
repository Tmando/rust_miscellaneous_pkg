initSidebarItems({"fn":[["get_default","Executes a closure with a reference to this thread’s current dispatcher."],["set_default","Sets the dispatch as the default dispatch for the duration of the lifetime of the returned DefaultGuard"],["set_global_default","Sets this dispatch as the global default for the duration of the entire program. Will be used as a fallback if no thread-local dispatch has been set in a thread (using `with_default`.)"],["with_default","Sets this dispatch as the default for the duration of a closure."]],"struct":[["DefaultGuard","A guard that resets the current default dispatcher to the prior default dispatcher when dropped."],["Dispatch","`Dispatch` trace data to a [`Subscriber`]."],["SetGlobalDefaultError","Returned if setting the global dispatcher fails."]]});