//! Options which can be passed to various `Docker` commands.

use url::form_urlencoded;

/// Options for `Docker::containers`.  This uses a "builder" pattern, so
/// most methods will consume the object and return a new one.
#[derive(Debug, Clone, Default)]
pub struct ContainerListOptions {
    all: bool,
    //before: Option<String>,
    //filter: Filter,
    latest: bool,
    limit: Option<u64>,
    //since: Option<String>,
    size: bool,
}

impl ContainerListOptions {
    /// Return all containers, including stopped ones.
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// Return just the most-recently-started container (even if it has
    /// stopped).
    pub fn latest(mut self) -> Self {
        self.latest = true;
        self
    }

    /// Limit the number of containers we return.
    pub fn limit(mut self, n: u64) -> Self {
        self.limit = Some(n);
        self
    }

    /// Calculate the total file sizes for our containers.  **WARNING:**
    /// This is very expensive.
    pub fn size(mut self) -> Self {
        self.size = true;
        self
    }

    /// Convert to URL parameters.
    pub fn to_url_params(&self) -> String {
        let mut params = form_urlencoded::Serializer::new(String::new());
        if self.all {
            params.append_pair("all", "1");
        }
        if self.latest {
            params.append_pair("latest", "1");
        }
        if let Some(limit) = self.limit {
            params.append_pair("limit", &limit.to_string());
        }
        if self.size {
            params.append_pair("size", "1");
        }
        params.finish()
    }
}

/// These options can be used for stopping or restarting a container.
pub struct ContainerStatusOptions {
    /// time until the status changes
    time: Option<u16>
}

impl ContainerStatusOptions {

    /// The time to wait before changing the status
    pub fn time(mut self, time: u16) -> Self {
        self.time = Some(time);
        self
    }

    pub fn to_url_params(&self) -> String {
        let mut params = form_urlencoded::Serializer::new(String::new());

        if let Some(time) = self.time {
            params.append_pair("t", &time.to_string());
        }
        params.finish()
    }
}

pub struct ContainerStartOptions {
    /// Custom key for deattaching from the container
    detach_keys: Option<String>
}

impl ContainerStartOptions {

    pub fn detach_keys(mut self, keys: String) -> Self {
        self.detach_keys = Some(keys);
        self
    }

    pub fn to_url_params(&self) -> String {
        let mut params = form_urlencoded::Serializer::new(String::new());

        if let &Some(ref keys) = &self.detach_keys {
            params.append_pair("detachKeys", &keys);
        }
        params.finish()
    }
}

pub struct ContainerKillOptions {
    /// What kind of POSIX signal to send to the container
    signal: Option<String>
}

impl ContainerKillOptions {

    /// the signal to send to the container
    pub fn signal(mut self, signal: String) -> Self {
        self.signal = Some(signal);
        self
    }

    pub fn to_url_params(&self) -> String {
        let mut params = form_urlencoded::Serializer::new(String::new());

        if let &Some(ref signal) = &self.signal {
            params.append_pair("signal", &signal);
        }
        params.finish()
    }
}
