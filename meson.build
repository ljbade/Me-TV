# -*- mode:python; coding:utf-8; -*-

project(
    'Me TV',
    'rust',
    #version: '', # This is not used due to the way things are built, so don't have it.
    license: 'GPLv3',
)

prefix = get_option('prefix')
bindir = get_option('bindir')
datadir = get_option('datadir')
applicationsdir = join_paths(datadir, 'applications')
iconsdir = join_paths(datadir, 'icons')

cargo = find_program('cargo')
cargo_script = find_program('scripts/cargo.sh')

sources = run_command('sh', '-c', 'cd ${MESON_SOURCE_ROOT} && ls src/*.rs').stdout().strip().split()

me_tv_target_name = 'me-tv'

me_tv = custom_target(
    me_tv_target_name,
    build_by_default: true,
    console: true,
    input: sources,
    output: [me_tv_target_name],
    install: true,
    install_dir: bindir,
    command: [cargo_script, '@CURRENT_SOURCE_DIR@', '@OUTPUT@']
)

me_tv_recording_target_name = me_tv_target_name + '-record'

me_tv_recording = custom_target(
    me_tv_recording_target_name,
    build_by_default: true,
    console: true,
    input: 'src/bin/' + me_tv_recording_target_name + '.rs',
    output: [me_tv_recording_target_name],
    install: true,
    install_dir: bindir,
    command: [cargo_script, '@CURRENT_SOURCE_DIR@', '@OUTPUT@']
)

me_tv_schedule_target_name = me_tv_target_name + '-schedule'

me_tv_schedule = custom_target(
    me_tv_schedule_target_name,
    build_by_default: true,
    console: true,
    input: 'src/bin/' + me_tv_schedule_target_name + '.rs',
    output: [me_tv_schedule_target_name],
    install: true,
    install_dir: bindir,
    command: [cargo_script, '@CURRENT_SOURCE_DIR@', '@OUTPUT@']
)

install_data('src/resources/images/me-tv.png', install_dir: iconsdir)
install_data('data/me-tv.desktop', install_dir: applicationsdir)
install_man('doc/me-tv.1')
