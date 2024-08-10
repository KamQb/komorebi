use crate::widget::BarWidget;
use sysinfo::RefreshKind;
use sysinfo::System;

pub struct Cpu {
    system: System,
    no_of_samples: i64,
    current_sample: i64,
    current_cpu_usage: f32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            system: System::new_with_specifics(RefreshKind::default()),
            no_of_samples: 100,
            current_cpu_usage: 0.0,
            current_sample: 0,
        }
    }
}

impl BarWidget for Cpu {
    fn output(&mut self) -> Vec<String> {
        if self.current_sample <= self.no_of_samples {
            self.current_sample += 1;
        } else {
            self.system.refresh_cpu();
            self.current_cpu_usage = self.system.global_cpu_info().cpu_usage();
            self.current_sample = 0;
        }

        vec![format!("CPU: {:.1}%", self.current_cpu_usage)]
    }
}
