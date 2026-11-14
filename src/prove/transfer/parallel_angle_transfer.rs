use crate::figure::{Figure, Object, Point};
use crate::prove::condition::{Condition, ParallelCondition, SumCondition};
use crate::prove::transfer::TransferType;
use crate::prove::transfer::base_transfer::BaseTransfer;
use crate::prove::transfer::parallel_facts::{
    CollinearSet, Config, SideMap, collinear_father, collect_collinear, collect_parallel,
    collect_sides, interior_info, match_config, points_on_line,
};
use crate::ratio::Ratio;

pub struct ParallelAngleTransfer {
    pub corresponding: bool,
    pub alternate_interior: bool,
    pub co_interior: bool,
}

#[derive(Clone, Copy)]
enum Kind {
    Corresponding,
    AlternateInterior,
    CoInterior,
}

impl Kind {
    fn transfer_type(self) -> TransferType {
        match self {
            Kind::Corresponding => TransferType::ParallelCorresponding,
            Kind::AlternateInterior => TransferType::ParallelAlternateInterior,
            Kind::CoInterior => TransferType::ParallelCoInterior,
        }
    }
}

impl BaseTransfer for ParallelAngleTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool {
        let parallels = collect_parallel(figure);
        let sides = collect_sides(figure);
        let collinear = collect_collinear(figure);
        let mut candidates = Vec::new();
        for parallel in &parallels {
            for side in sides.values() {
                let Some(cfg) = match_config(parallel, side) else { continue };
                let on_line = points_on_line(cfg.p, cfg.q, &collinear);
                for &xt in &on_line {
                    if xt == cfg.p {
                        continue;
                    }
                    for &yt in &on_line {
                        if yt == cfg.q {
                            continue;
                        }
                        if let Some((cond, fathers, kind)) =
                            self.derive(parallel, &cfg, xt, yt, &sides, &collinear)
                        {
                            candidates.push((cond, fathers, kind.transfer_type()));
                        }
                    }
                }
            }
        }
        let mut changed = false;
        for (cond, fathers, describe) in candidates {
            changed |= figure.add_derived(cond, fathers, describe);
        }
        changed
    }
}

impl ParallelAngleTransfer {
    fn derive(
        &self,
        parallel: &ParallelCondition,
        cfg: &Config,
        xt: Point,
        yt: Point,
        sides: &SideMap,
        collinear: &CollinearSet,
    ) -> Option<(Condition, Vec<Condition>, Kind)> {
        let (ip, ip_father) = interior_info(xt, cfg.q, parallel.a, sides)?;
        let (iq, iq_father) = interior_info(yt, cfg.p, parallel.b, sides)?;
        let kind = self.classify(ip, iq, cfg.same)?;

        let ang_p = Object::angle(cfg.x, cfg.p, xt);
        let ang_q = Object::angle(cfg.y, cfg.q, yt);
        let cond = match kind {
            Kind::CoInterior => Condition::Sum(SumCondition::new(
                vec![ang_p, ang_q],
                vec![],
                Ratio::from(180),
            )),
            _ => Condition::Sum(SumCondition::new(vec![ang_p], vec![ang_q], Ratio::from(0))),
        };

        let mut fathers = vec![
            Condition::Parallel(parallel.clone()),
            Condition::Side(cfg.side),
        ];
        if let Some(s) = ip_father {
            fathers.push(Condition::Side(s));
        }
        if let Some(s) = iq_father {
            fathers.push(Condition::Side(s));
        }
        if let Some(c) = collinear_father(cfg.p, cfg.q, xt, collinear) {
            fathers.push(c);
        }
        if let Some(c) = collinear_father(cfg.p, cfg.q, yt, collinear) {
            fathers.push(c);
        }
        Some((cond, fathers, kind))
    }

    fn classify(&self, ip: bool, iq: bool, same: bool) -> Option<Kind> {
        let kind = if ip && iq {
            if same { Kind::CoInterior } else { Kind::AlternateInterior }
        } else if ip != iq && same {
            Kind::Corresponding
        } else {
            return None;
        };
        let enabled = match kind {
            Kind::CoInterior => self.co_interior,
            Kind::AlternateInterior => self.alternate_interior,
            Kind::Corresponding => self.corresponding,
        };
        if enabled { Some(kind) } else { None }
    }
}