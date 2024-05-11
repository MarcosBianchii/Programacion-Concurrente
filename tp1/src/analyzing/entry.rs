//! Entry struct models a question entry in the data for a certain site.

use serde::Deserialize;

use std::str::FromStr;

#[derive(Deserialize)]
pub struct Entry {
    texts: [String; 2],
    tags: Vec<String>,
}

impl FromStr for Entry {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Entry {
    pub fn consume(self) -> (usize, Vec<String>) {
        let Self {
            texts: [title, body],
            tags,
        } = self;

        let (title_ws, body_ws) = rayon::join(
            || title.split_whitespace().count(),
            || body.split_whitespace().count(),
        );

        (title_ws + body_ws, tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json() {
        let s = r#"{"texts": ["How to filter text matched by global command?", "I have a log file full of arbitrary lines, interspersed with some lines which are a proper xml sequence, with all the tags on a single line (for each sequence).  I want to simply process this so that all the xml lines become broken out in easy to read format.  There is a utility in linux called xmllint that looks useful here.\n\nI am able to do this easily manually in vim, by searching for \n\n!! xmllint --format --recover - \n\n\nInterestingly, \n\n:! cmd \n\n\ndoesn't work here, but\n\n:.! cmd \n\n\ndoes.  I'm not sure if that's because the first one is legacy \"Ex\" mode, while adding the range (.) makes the second one a Vim filter?\n\nWhat I want is to be able to do it automatically, not by saving manual commands in a recorded register and replaying it possibly tens or hundreds of times, but O(1) commands to simply find all the lines with \"xml\" and filter them through this external command, replacing their text in the buffer with the nicely formatted version.\n\nThis should definitely be possible given the power and style of vim, but I've tried for hours and can't do it.  \n\nThe idea is something like\n\n:%g/xml/normal!!:$xmllint --format --recover -\n\n\nI've tried with and without normal, and all the variations of : and ! to try to filter through an external command.\n\nMost of the time, it does absolutely nothing to the buffer, nothing even to undo, even though the :g part is definitely matching.\nOther times, like\n\n:%g/xml/!xmllint --format --recover -\n\n\nit waits forever until I send a Ctrl-C for each match.  Basically it seems to be waiting for xmllint which is stuck waiting at STDIN.\n\nHow can I take a working :%g/pattern/ command and make it replace the matching lines with the output of an external command (a command which itself takes the aforementioned matching line as its input) ?"], "tags": ["external-command", "linux", "global-command"]}"#;

        assert!(Entry::from_str(s).is_ok());
    }
}
