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
    sink: Arc<Sink>,
    current_source: Option<Arc<File>>,
    _stream: Option<Arc<SafeOutputStream>>,
}

#[pymethods]
impl Player {
    #[new]
    pub fn new() -> PyResult<Self> {
        let (stream, stream_handle) = to_pyException!(OutputStream::try_default())?;
        let sink = Arc::new(to_pyException!(Sink::try_new(&stream_handle.clone()))?);
        Ok(Self {
            sink,
            current_source: None,
            _stream: Some(Arc::new(SafeOutputStream(stream))),
        })
    }

    pub fn play(&mut self, path: &str) -> PyResult<()> {
        let file = Arc::new(File::open(path)?);
        self.current_source = Some(file.clone());
        let source = to_pyException!(decoder::Decoder::new(file))?;
        self.sink.append(source);
        Ok(())
    }

    pub fn get_duration(&self) -> PyResult<Option<u128>> {
        if let Some(source) = self.current_source.clone() {
            let duration = to_pyException!(decoder::Decoder::new(source))?.total_duration();
            return Ok(duration.map(|d| d.as_millis()));
        }
        Ok(None)
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }

    pub fn pause(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn seek(&self, pos: u64) -> PyResult<()> {
        let duration = Duration::from_millis(pos);
        to_pyException!(self.sink.try_seek(duration))?;
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
