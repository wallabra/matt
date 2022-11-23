use matt::prelude::*;

fn initial_buffer() -> StereoBuffer {
    let mut buf = StereoBuffer::new(2);

    buf.left[0] = 2;
    buf.left[1] = 100;
    buf.right[0] = 6000;
    buf.right[1] = 14;

    buf
}

#[cfg(test)]
mod tests {
    use super::initial_buffer;
    use matt::prelude::*;

    #[test]
    fn test_buffer_copy() {
        let buf = initial_buffer();
        let mut buf2 = StereoBuffer::new(2);

        buf.copy_to_and_resample((&mut buf2).into(), Box::from(NoResample));

        assert_eq!(buf.left[0], buf2.left[0]);
        assert_eq!(buf.left[1], buf2.left[1]);
        assert_eq!(buf.right[0], buf2.right[0]);
        assert_eq!(buf.right[1], buf2.right[1]);
    }

    #[test]
    fn test_buffer_resample_none() {
        let buf = initial_buffer();
        let mut buf2 = StereoBuffer::new(6);

        buf.copy_to_and_resample((&mut buf2).into(), Box::from(NoResample));

        for (idx, val) in buf.left.iter().enumerate() {
            assert_eq!(*val, buf2.left[idx * 3]);
            assert_eq!(*val, buf2.left[idx * 3 + 1]);
            assert_eq!(*val, buf2.left[idx * 3 + 2]);
        }

        for (idx, val) in buf.right.iter().enumerate() {
            assert_eq!(*val, buf2.right[idx * 3]);
            assert_eq!(*val, buf2.right[idx * 3 + 1]);
            assert_eq!(*val, buf2.right[idx * 3 + 2]);
        }
    }
}
