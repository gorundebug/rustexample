// Dashboard: Tokio Runtime
//
// Source: the stable RuntimeMonitor API from the official tokio-metrics crate.
// Metrics requiring Tokio's tokio_unstable cfg are intentionally excluded.

local g = import 'github.com/grafana/grafonnet/gen/grafonnet-v11.0.0/main.libsonnet';
local lib = import '_lib.libsonnet';

local jobFilter = 'job=~"$job"';

lib.dashboard(
  title='%s / Tokio Runtime' % lib.svc,
  uid='%s-runtime' % lib.svc,
  tags=['runtime', 'rust', 'tokio'],
  variables=[
    lib.dsVar,
    lib.jobVar('tokio_workers_count'),
  ],
  panels=[
    lib.row('Runtime'),
    lib.ts(
      title='Workers and Live Tasks',
      targets=[
        lib.promQ('tokio_workers_count{%s}' % jobFilter, 'workers {{job}}'),
        lib.promQ('tokio_live_tasks_count{%s}' % jobFilter, 'live tasks {{job}}'),
      ],
      w=12, h=8,
      unit='short',
    ),
    lib.ts(
      title='Worker Busy Ratio',
      targets=[lib.promQ('tokio_busy_ratio{%s}' % jobFilter, '{{job}}')],
      w=12, h=8,
      unit='percentunit',
    ),
    lib.ts(
      title='Global Queue Depth',
      targets=[lib.promQ('tokio_global_queue_depth{%s}' % jobFilter, '{{job}}')],
      w=12, h=8,
      unit='short',
    ),
    lib.ts(
      title='Worker Busy Duration per Sampling Interval',
      targets=[
        lib.promQ('tokio_total_busy_duration_seconds{%s}' % jobFilter, 'total {{job}}'),
        lib.promQ('tokio_max_busy_duration_seconds{%s}' % jobFilter, 'max {{job}}'),
        lib.promQ('tokio_min_busy_duration_seconds{%s}' % jobFilter, 'min {{job}}'),
      ],
      w=12, h=8,
      unit='s',
    ),
    lib.row('Worker Parking'),
    lib.ts(
      title='Worker Parks per Sampling Interval',
      targets=[
        lib.promQ('tokio_total_park_count{%s}' % jobFilter, 'total {{job}}'),
        lib.promQ('tokio_max_park_count{%s}' % jobFilter, 'max {{job}}'),
        lib.promQ('tokio_min_park_count{%s}' % jobFilter, 'min {{job}}'),
      ],
      w=24, h=8,
      unit='short',
    ),
  ]
)
