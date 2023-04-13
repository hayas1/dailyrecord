use super::view::{CalendarProps, CalendarViewProps};
use chrono::{Duration, NaiveDate};

// TODO remove
pub struct Calendar {
    pub scale: Scale,
    pub inducing: NaiveDate,
}
impl From<Calendar> for CalendarProps {
    fn from(value: Calendar) -> Self {
        CalendarViewProps::from(value).calendar_props
    }
}
impl From<Calendar> for CalendarViewProps {
    fn from(value: Calendar) -> Self {
        let now = crate::supply::now().into();
        let Calendar { inducing, scale } = value;
        let calendar_props = CalendarProps { now, inducing };
        Self { scale, calendar_props }
    }
}

impl From<CalendarViewProps> for Calendar {
    fn from(value: CalendarViewProps) -> Self {
        let CalendarViewProps { scale, calendar_props } = value;
        let CalendarProps { inducing, .. } = calendar_props;
        Self { inducing, scale }
    }
}
impl Calendar {
    // TODO trait ? (with some arguments for performance)
    pub fn to_props(&self) -> CalendarViewProps {
        let Calendar { scale, inducing, .. } = self;
        let (scale, inducing) = (scale.clone(), inducing.clone()); // TODO better clone solution...?
        Calendar { scale, inducing }.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Year,
    Month,
    Week,
    Day,
}
impl Default for Scale {
    fn default() -> Self {
        Self::Week
    }
}
impl Scale {
    pub fn prev(&self, &inducing: &NaiveDate) -> NaiveDate {
        match self {
            Self::Year => todo!("`Duration::years` is not implemented?"),
            Self::Month => todo!("`Duration::months` is not implemented?"),
            Self::Week => inducing - Duration::weeks(1),
            Self::Day => inducing - Duration::days(1),
        }
    }
    pub fn next(&self, &inducing: &NaiveDate) -> NaiveDate {
        match self {
            Self::Year => todo!("`Duration::years` is not implemented?"),
            Self::Month => todo!("`Duration::months` is not implemented?"),
            Self::Week => inducing + Duration::weeks(1),
            Self::Day => inducing + Duration::days(1),
        }
    }
}
