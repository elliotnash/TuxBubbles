tuxbubbles_sources = [
  'main.vala',
  'application.vala',
  'window.vala',
]

blueprints = custom_target('blueprints',
  input: files(
    'shortcuts-dialog.blp',
    'window.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

tuxbubbles_deps = [
  config_dep,
  dependency('gtk4'),
  dependency('libadwaita-1', version: '>= 1.4'),
]

tuxbubbles_sources += gnome.compile_resources('tuxbubbles-resources',
  'tuxbubbles.gresource.xml',
  dependencies: blueprints,
  c_name: 'tuxbubbles'
)

executable('tuxbubbles', tuxbubbles_sources,
  dependencies: tuxbubbles_deps,
  include_directories: config_inc,
       install: true,
)
