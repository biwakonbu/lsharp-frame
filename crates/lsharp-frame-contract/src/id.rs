use std::fmt;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub u64);

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

id_type!(UiNodeId);
id_type!(EffectId);
id_type!(PtySessionId);
id_type!(ProcessId);
id_type!(SurfaceId);
id_type!(ArtifactId);
id_type!(DisplayListId);
