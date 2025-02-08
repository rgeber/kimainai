# KimaiNai

A CLI client for Kimai time tracking designed for people with strong aversions to time tracking and or Kimai.

## Build and install

At this stage I haven't set up any build pipelines (lazy, sorry). But building it is quite easy:

* Make sure you have Rust installed: https://www.rust-lang.org/tools/install
* Enter the project directory and run `cargo build -r`
* The binary you want will be `./target/release/kimainai`
* Enjoy

## Motivation

I don't work by the hour, ever. However some projects may still need entries into their time tracking tools (in this case that's Kimai which is a decent tool). Yet instead of fighting my way through the UI entering a few months worth of work times that are essentially exactly the same every day and hating every second of this process I fired up my editor instead.

Spending a bit of time sharpening my fledling Rust skills and gaining experience with the chrono library seemed to a much better use of my time and certainly more enjoyable.

Kimainai is by no means complete nor is it meant to be a polished product. It's a hack fixing a particuluarily unpeasent part of my work life that works for me. If you find it useful then all the better. Feel free to add features you might need.

## Set a Kimai API Password

Even so newer versions of Kimai seem of favour a more reasonable API key setup this version is only compatible / tested with the depricated API password as this is the setup I built this tool for.

To set the password navigate to the `API Access` tab in your user profile's settings. On the bottom of the page you'll find a `Password` setting. Use it to set an API password.

Once the password is set add it to your `~/.kimainai.toml` file (or equivalent).

## Config

The default config file path is `~/.kimainai.toml` and looks like this:

```toml
[api]
url="https://track-my.time.tld/api"
user="employee_of_the_month"
key="i_promise_to_change_this"

[workday]
break_hour=11
break_minute=30
break_duration=60
start_hour=7
start_minute=30
duration_hours=8
duration_minutes=480

[defaults]
project_id=13
activity_id=37
```

Some of these values stand in as default values for some CLI parameters if they aren't set specifically.

## Usage

### Getting information

The following commands allow you to query information generally needed to file timesheets:

* `get-me` shows information about the user you're authenticated as. Useful to see if API access is set up correctly.
* `list-customers` - list all customers. The interesting part may be the ID. However no customer ID is used in a write function kimainai provides at this point.
* `list-projects` - list of projects. You'll need the right project ID to file timehsheets.
* `list-activities` - list of activities. Ideally paired with `-p <PROJECT_ID>` to limit the output to the project you care about as there can be a lot of duplicates in the list.

### File a single workday

This will file a single work day including break time if required.

For example:

```bash
kimainai file-workday \
   --project-id 13 --activity-id 37 \
   --break-start-hour 11 --break-start-minute 30 --break-duration 60 \
   -Y 2024 -M 12 -d 24 -H 7 -i 30 --duration-hours 8
```

This example will file two timesheets for Dec. 24 2004 from 7:30 until 11:30 (break time) followed by another timesheet from 12:30 (break end) to 16:30 (8 hour duration complete).

You may define values that rarely change in the config file as described above.

Setting `--break-duration 0` will elimniate the break completely and just file a single time sheet from the start time to the calculated end fime based on the duration value.

Easy :)

### Bulk file workdays

Bulk filing is **the** reason I made kimainai. This is just a wrapper of the sinlge filing method described above. The main difference is that you can define a first and a last day as well as excluding weekdays (your days off).

For example:

```bash
kimainai bulk-file-workdays --project-id 13 --activity-id 37 --break-duration 60 \
    -y 2025 -m 2 -d 7 -Y 2025 -M 2 -D 20 -e fri,sat
```

This example creates timesheets from Feb 2, 2025 until Feb 20 2025 but exclude all Fridays and Saturdays.

Other supporting values are read from the config file (e.g. start time, etc). Use `kimainai bulk-file-workdays --help` to see the full range of arguments.

## Troubleshooting

Nothing yet ... if you run into issues please open an issue.

## Known issues

* The tool may not be very good at dealing with entries spanning multiple days
* There are **NO** tests. I was lazy ... sorry.
* Testes only on Linux. Windows and MacOS users may experience issues. However nothing should prevent usage on any platform you choose.
* No binary pipeline yet. *TODO*
* No official publication (e.g. to use `cargo install`). *TODO*
