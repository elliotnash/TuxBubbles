tuxbubbles_sources = [
  'app/main.vala',
  'app/application.vala',
  'ui/window.vala',
]

blueprints = custom_target('blueprints',
  input: files(
    'ui/shortcuts-dialog.blp',
    'ui/window.blp',
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'), 'batch-compile',
    meson.current_build_dir() / 'ui',              # output directory for generated .ui files
    meson.current_source_dir() / 'ui',             # base directory for sources
    '@INPUT@'
  ],
)

tuxbubbles_deps = [
  config_dep,
  dependency('gtk4'),
  dependency('libadwaita-1', version: '>= 1.4'),
]

tuxbubbles_sources += gnome.compile_resources('tuxbubbles-resources',
  'ui/tuxbubbles.gresource.xml',
  dependencies: blueprints,
  c_name: 'tuxbubbles'
)

executable('tuxbubbles', tuxbubbles_sources,
  dependencies: tuxbubbles_deps,
  include_directories: config_inc,
       install: true,
)
