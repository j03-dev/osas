use std::{fs::File, sync::Arc, time::Duration};

use pyo3::{exceptions::PyException, prelude::*};
use rodio::{decoder, OutputStream, Sink, Source};

macro_rules! to_pyException {
    ($result:expr) => {
        $result.map_err(|err| PyException::new_err(err.to_string()))
    };
}

#[allow(dead_code)]
struct SafeOutputStream(OutputStream);

unsafe impl Send for SafeOutputStream {}
unsafe impl Sync for SafeOutputStream {}

#[pyclass]
struct Player {
    sink: Sink,
    _stream: Option<Arc<SafeOutputStream>>,
}

#[pymethods]
impl Player {
    #[new]
    pub fn new() -> PyResult<Self> {
        let (stream, stream_handle) = to_pyException!(OutputStream::try_default())?;

        let sink = to_pyException!(Sink::try_new(&stream_handle.clone()))?;

        Ok(Self {
            sink,
            _stream: Some(Arc::new(SafeOutputStream(stream))),
        })
    }

    pub fn play(&self, path: &str) -> PyResult<u128> {
        let file = File::open(path)?;
        let source = to_pyException!(decoder::Decoder::new(file))?;
        let duration = source.total_duration().unwrap();
        self.sink.append(source);
        Ok(duration.as_millis())
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn seek(&self, pos: Duration) -> PyResult<()> {
        to_pyException!(self.sink.try_seek(pos))?;
        Ok(())
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn get_pos(&self) -> u128 {
        self.sink.get_pos().as_millis()
    }
}

#[pymodule]
fn osas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Player>()?;
    Ok(())
}
