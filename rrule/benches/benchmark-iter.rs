use chrono::{DateTime, TimeZone};
use criterion::{criterion_group, criterion_main, Criterion};
use rrule::{Frequency, RRule, RRuleSet, Tz};

fn ymd_hms(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DateTime<Tz> {
    Tz::Tz(chrono_tz::Tz::UTC)
        .with_ymd_and_hms(year, month, day, hour, minute, second)
        .unwrap()
}

fn frequency(c: &mut Criterion) {
    let mut group = c.benchmark_group("frequency");
    group.throughput(criterion::Throughput::Elements(1));

    let dt_start = ymd_hms(1960, 1, 4, 9, 0, 0);

    group.bench_function("secondly", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Secondly,
                count: None,
                interval: 2,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("minutely", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Minutely,
                count: None,
                interval: 5,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("hourly", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Hourly,
                count: None,
                interval: 2,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("daily", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Daily,
                count: None,
                interval: 1,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("weekly", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Weekly,
                count: None,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("daily+hourly", |b| {
        let mut iter = {
            let daily = RRule {
                freq: Frequency::Daily,
                count: None,
                interval: 1,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();

            let hourly = RRule {
                freq: Frequency::Hourly,
                count: None,
                interval: 2,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();

            RRuleSet::new(dt_start)
                .rrule(daily)
                .rrule(hourly)
                .into_iter()
        };

        b.iter(|| iter.next())
    });

    group.finish();
}

fn filter_by_xx(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter");
    group.throughput(criterion::Throughput::Elements(1));

    let dt_start = ymd_hms(1960, 1, 4, 9, 0, 0);

    group.bench_function("secondly by hour", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Secondly,
                count: None,
                by_hour: vec![9],
                interval: 2,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("minutely by day", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Minutely,
                count: None,
                by_weekday: vec![rrule::NWeekday::Every(rrule::Weekday::Mon)],
                interval: 5,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("hourly by month day", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Hourly,
                count: None,
                by_month_day: vec![-1],
                interval: 2,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("daily by month", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Daily,
                count: None,
                by_weekday: vec![rrule::NWeekday::Every(rrule::Weekday::Mon)],
                by_month: vec![1],
                interval: 1,
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.bench_function("weekly by month day", |b| {
        let mut iter = {
            let rrule = RRule {
                freq: Frequency::Weekly,
                count: None,
                by_month_day: vec![-1],
                ..Default::default()
            }
            .validate(dt_start)
            .unwrap();
            RRuleSet::new(dt_start).rrule(rrule).into_iter()
        };

        b.iter(|| iter.next())
    });

    group.finish();
}

criterion_group!(benches, frequency, filter_by_xx);
criterion_main!(benches);
