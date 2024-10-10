use crate::{prelude::*, unit::combat::DamageEvent};

use super::{stats::Damage, SpellEvent, SpellImpactEvent, SpellTarget};

pub(crate) fn plugin<T: SpellEffectConfiguration + Component>(app: &mut App) {
    T::configure(app);
}

pub(crate) trait SpellEffectConfiguration: Component + 'static {
    fn configure(app: &mut App);
}

#[derive(Component)]
pub(crate) struct DealDamage;

#[spell_effect]
impl DealDamage {
    #[on(SpellImpactEvent)]
    fn on_impact(
        In(event): In<SpellEvent<SpellImpactEvent>>,
        with_damage: Query<&Damage>,
        mut event_writer: EventWriter<DamageEvent>,
    ) {
        let event = event.event;
        let damage = or_return!(with_damage.get(event.spell));
        if let SpellTarget::Entity(target) = event.target {
            event_writer.send(DamageEvent {
                target,
                damage: **damage,
                source: event.caster,
            });
        };
    }
}
