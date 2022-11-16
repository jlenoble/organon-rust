use crate::Timer;
use super::Context;

impl Context {
    ////////////////////////////////////////////////////////////////////////////////
    // Support verbosity levels:
    //
    //   rc.verbose=1          Show all feedback.
    //   rc.verbose=0          Show regular feedback.
    //   rc.verbose=nothing    Show the absolute minimum.
    //   rc.verbose=one,two    Show verbosity for 'one' and 'two' only.
    //
    // TODO This mechanism is clunky, and should slowly evolve into something more
    //      logical and consistent.  This should probably mean that 'nothing' should
    //      take the place of '0'.

    pub fn verbose(&mut self, token: &str) -> bool {
        if self.verbosity.is_empty() {
            self.verbosity_legacy = self.config.get_boolean("verbose", None);
            let tokens = self.config.get("verbose", None);
            for token in tokens.split(',') {
                self.verbosity.insert(token.to_owned());
            }

            // Regular feedback means almost everything.
            // This odd test is to see if a Boolean-false value is a real one, which
            // means it is not 1/true/T/yes/on, but also should not be one of the
            // valid tokens either.
            if !self.verbosity_legacy && !self.verbosity.is_empty() {
                let v = self.verbosity.iter().next().unwrap();
                if
                    v != "nothing" &&
                    v != "affected" && // This list must be complete.
                    v != "blank" && //
                    v != "context" && //
                    v != "default" && //
                    v != "edit" && //
                    v != "filter" && //
                    v != "footnote" && //
                    v != "header" && //
                    v != "label" && //
                    v != "new-id" && //
                    v != "new-uuid" && //
                    v != "override" && //
                    v != "project" && //
                    v != "recur" && //
                    v != "special" && //
                    v != "sync"
                {
                    // This list emulates rc.verbose=off in version 1.9.4.
                    self.verbosity.insert("blank".to_owned());
                    self.verbosity.insert("label".to_owned());
                    self.verbosity.insert("new-id".to_owned());
                    self.verbosity.insert("edit".to_owned());
                }
            }

            // Some flags imply "footnote" verbosity being active.  Make it so.
            // TODO: Some of these may not use footnotes yet.  They should.
            for flag in ["affected", "new-id", "new-uuid", "project", "override", "recur"] {
                if self.verbosity.contains(flag) {
                    self.verbosity.insert("footnote".to_owned());
                    break;
                }
            }

            // Some flags imply "header" verbosity being active.  Make it so.
            if self.verbosity.contains("default") {
                self.verbosity.insert("header".to_owned());
            }
        }

        // rc.verbose=true|y|yes|1|on overrides all.
        if self.verbosity_legacy {
            return true;
        }

        // rc.verbose=nothing overrides all.
        if self.verbosity.len() == 1 {
            if let Some(item) = self.verbosity.iter().next() {
                return item != "nothing";
            }
        }

        // Specific token match.
        if self.verbosity.contains(token) {
            return true;
        }

        false
    }

    ////////////////////////////////////////////////////////////////////////////////
    // No duplicates.
    pub fn header(&mut self, input: &String) {
        if !input.is_empty() && self.headers.contains(input) {
            self.headers.push(input.to_owned());
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub fn debug_timing(details: &String, timer: &Timer) {
        println!("Timer {details} {} sec", timer.total_s());
    }
}