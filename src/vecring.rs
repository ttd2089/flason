pub struct Buffer {
    ring: Vec<Vec<u8>>,
    read_cursor: Position,
    write_cursor: Position,
}

impl Buffer {
    fn with_options(opts: &Options) -> Self {
        let mut ring = Vec::with_capacity(opts.ring_capacity);
        ring.push(Vec::with_capacity(opts.buffer_size));
        Self {
            ring: ring,
            read_cursor: Position::default(),
            write_cursor: Position::default(),
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::with_options(&Options::default())
    }
}

struct Options {
    ring_capacity: usize,
    buffer_size: usize,
}

impl Options {
    const DEFAULT_INITIAL_RING_CAPACITY: usize = 64;
    const INITIAL_BUFFER_SIZE: usize = 4;
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ring_capacity: Self::DEFAULT_INITIAL_RING_CAPACITY,
            buffer_size: Self::INITIAL_BUFFER_SIZE,
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    // segment is declared first so that lexical ordering will give us the desired comparisons.
    segment: usize,
    offset: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            segment: 0,
            offset: 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_default() {
        let d = Position::default();
        assert_eq!(d.segment, 0);
        assert_eq!(d.offset, 0);
    }

    #[test]
    fn test_position_comparisons() {
        assert_eq!(
            Position {
                segment: 0,
                offset: 0
            },
            Position {
                segment: 0,
                offset: 0
            }
        );
        assert_eq!(
            Position {
                segment: 4,
                offset: 2
            },
            Position {
                segment: 4,
                offset: 2
            }
        );

        assert_ne!(
            Position {
                segment: 0,
                offset: 1
            },
            Position {
                segment: 1,
                offset: 0
            }
        );

        assert!(
            Position {
                segment: 0,
                offset: 1
            } < Position {
                segment: 0,
                offset: 2
            }
        );
        assert!(
            Position {
                segment: 0,
                offset: 1
            } < Position {
                segment: 1,
                offset: 1
            }
        );
        assert!(
            Position {
                segment: 0,
                offset: 1
            } < Position {
                segment: 1,
                offset: 0
            }
        );

        assert!(
            Position {
                segment: 0,
                offset: 1
            } <= Position {
                segment: 0,
                offset: 2
            }
        );
        assert!(
            Position {
                segment: 0,
                offset: 1
            } <= Position {
                segment: 1,
                offset: 1
            }
        );
        assert!(
            Position {
                segment: 0,
                offset: 1
            } <= Position {
                segment: 1,
                offset: 0
            }
        );
        assert!(
            Position {
                segment: 4,
                offset: 2
            } <= Position {
                segment: 4,
                offset: 2
            }
        );

        assert!(
            Position {
                segment: 1,
                offset: 1
            } > Position {
                segment: 1,
                offset: 0
            }
        );
        assert!(
            Position {
                segment: 1,
                offset: 1
            } > Position {
                segment: 0,
                offset: 1
            }
        );
        assert!(
            Position {
                segment: 1,
                offset: 1
            } > Position {
                segment: 0,
                offset: 2
            }
        );

        assert!(
            Position {
                segment: 1,
                offset: 1
            } >= Position {
                segment: 1,
                offset: 0
            }
        );
        assert!(
            Position {
                segment: 1,
                offset: 1
            } >= Position {
                segment: 0,
                offset: 1
            }
        );
        assert!(
            Position {
                segment: 1,
                offset: 1
            } >= Position {
                segment: 0,
                offset: 2
            }
        );
        assert!(
            Position {
                segment: 4,
                offset: 2
            } >= Position {
                segment: 4,
                offset: 2
            }
        );
    }
}
