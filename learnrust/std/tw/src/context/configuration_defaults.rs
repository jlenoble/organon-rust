////////////////////////////////////////////////////////////////////////////////
// This string is parsed and used as default values for configuration.
pub static CONFIGURATION_DEFAULTS: &str =
    "
# Taskwarrior program configuration file.
# For more documentation, see https://taskwarrior.org or try 'man task', 'man task-color',
# 'man task-sync' or 'man taskrc'

# Here is an example of entries that use the default, override and blank values
#   variable=foo   -- By specifying a value, this overrides the default
#   variable=      -- By specifying no value, this means no default
#   #variable=foo  -- By commenting out the line, or deleting it, this uses the default

# You can also refence environment variables:
#   variable=$HOME/task
#   variable=$VALUE

# Use the command 'task show' to see all defaults and overrides

# Files
data.location=~/.task
locking=1                                      # Use file-level locking
gc=1                                           # Garbage-collect data files - DO NOT CHANGE unless you are sure
exit.on.missing.db=0                           # Whether to exit if ~/.task is not found
hooks=1                                        # Master control switch for hooks

# Terminal
detection=1                                    # Detects terminal width
defaultwidth=80                                # Without detection, assumed width
defaultheight=24                               # Without detection, assumed height
avoidlastcolumn=0                              # Fixes Cygwin width problem
hyphenate=1                                    # Hyphenates lines wrapped on non-word-breaks
#editor=vi                                     # Preferred text editor
reserved.lines=1                               # Assume a 1-line prompt

# Miscellaneous
# verbose=                                     # Comma-separated list.  May contain any subset of:
# affected,blank,context,default,edit,filter,footnote,header,label,new-id,new-uuid,override,project,recur,special,sync
verbose=affected,blank,context,edit,header,footnote,label,new-id,project,special,sync,override,recur
confirmation=1                                 # Confirmation on delete, big changes
recurrence=1                                   # Enable recurrence
recurrence.confirmation=prompt                 # Confirmation for propagating changes among recurring tasks (yes/no/prompt)
allow.empty.filter=1                           # An empty filter gets a warning and requires confirmation
indent.annotation=2                            # Indent spaces for annotations
indent.report=0                                # Indent spaces for whole report
row.padding=0                                  # Left and right padding for each row of report
column.padding=1                               # Spaces between each column in a report
bulk=3                                         # 3 or more tasks considered a bulk change and is confirmed
nag=You have more urgent tasks.                # Nag message to keep you honest                      // TODO
search.case.sensitive=1                        # Setting to no allows case insensitive searches
active.indicator=*                             # What to show as an active task indicator
tag.indicator=+                                # What to show as a tag indicator
dependency.indicator=D                         # What to show as a dependency indicator
recurrence.indicator=R                         # What to show as a task recurrence indicator
recurrence.limit=1                             # Number of future recurring pending tasks
undo.style=side                                # Undo style - can be 'side', or 'diff'
regex=1                                        # Assume all search/filter strings are regexes
xterm.title=0                                  # Sets xterm title for some commands
expressions=infix                              # Prefer infix over postfix expressions
json.array=1                                   # Enclose JSON output in [ ]
abbreviation.minimum=2                         # Shortest allowed abbreviation
news.version=                                  # Latest version highlights read by the user

# Dates
dateformat=Y-M-D                               # Preferred input and display date format
dateformat.holiday=YMD                         # Preferred input date format for holidays
dateformat.edit=Y-M-D H:N:S                    # Preferred display date format when editing
dateformat.info=Y-M-D H:N:S                    # Preferred display date format for information
dateformat.report=                             # Preferred display date format for reports
dateformat.annotation=                         # Preferred display date format for annotations
date.iso=1                                     # Enable ISO date support
weekstart=sunday                               # Sunday or Monday only
displayweeknumber=1                            # Show week numbers on calendar
due=7                                          # Task is considered due in 7 days

# Calendar controls
calendar.legend=1                              # Display the legend on calendar
calendar.details=sparse                        # Calendar shows information for tasks w/due dates: full, sparse or none
calendar.details.report=list                   # Report to use when showing task information in cal
calendar.offset=0                              # Apply an offset value to control the first month of the calendar
calendar.offset.value=-1                       # The number of months the first month of the calendar is moved
calendar.holidays=none                         # Show public holidays on calendar:full, sparse or none
#calendar.monthsperline=3                      # Number of calendar months on a line

# Journal controls
journal.time=0                                 # Record start/stop commands as annotation
journal.time.start.annotation=Started task     # Annotation description for the start journal entry
journal.time.stop.annotation=Stopped task      # Annotation description for the stop  journal entry
journal.info=1                                 # Display task journal with info command

# Dependency controls
dependency.reminder=1                          # Nags on dependency chain violations
dependency.confirmation=1                      # Should dependency chain repair be confirmed?

# Urgency Coefficients
urgency.user.tag.next.coefficient=15.0         # Urgency coefficient for 'next' special tag
urgency.due.coefficient=12.0                   # Urgency coefficient for due dates
urgency.blocking.coefficient=8.0               # Urgency coefficient for blocking tasks
urgency.active.coefficient=4.0                 # Urgency coefficient for active tasks
urgency.scheduled.coefficient=5.0              # Urgency coefficient for scheduled tasks
urgency.age.coefficient=2.0                    # Urgency coefficient for age
urgency.annotations.coefficient=1.0            # Urgency coefficient for annotations
urgency.tags.coefficient=1.0                   # Urgency coefficient for tags
urgency.project.coefficient=1.0                # Urgency coefficient for projects
urgency.blocked.coefficient=-5.0               # Urgency coefficient for blocked tasks
urgency.waiting.coefficient=-3.0               # Urgency coefficient for waiting status
urgency.inherit=0                              # Recursively inherit highest urgency value from blocked tasks
urgency.age.max=365                            # Maximum age in days

#urgency.user.project.foo.coefficient=5.0      # Urgency coefficients for 'foo' project
#urgency.user.tag.foo.coefficient=5.0          # Urgency coefficients for 'foo' tag
#urgency.uda.foo.coefficient=5.0               # Urgency coefficients for UDA 'foo'

# Color controls.
color=1                                        # Enable color

# Here is the rule precedence order, highest to lowest.
# Note that these are just the color rule names, without the leading 'color.'
#      and any trailing '.value'.
rule.precedence.color=deleted,completed,active,keyword.,tag.,project.,overdue,scheduled,due.today,due,blocked,blocking,recurring,tagged,uda.

# General decoration
rule.color.merge=1
color.label=
color.label.sort=
color.alternate=on gray2
color.header=color3
color.footnote=color3
color.warning=bold red
color.error=white on red
color.debug=color4

# Task state
color.completed=
color.deleted=
color.active=rgb555 on rgb410
color.recurring=rgb013
color.scheduled=on rgb001
color.until=
color.blocked=white on color8
color.blocking=black on color15

# Project
color.project.none=

# Priority UDA
color.uda.priority.H=color255
color.uda.priority.L=color245
color.uda.priority.M=color250

# Tags
color.tag.next=rgb440
color.tag.none=
color.tagged=rgb031

# Due
color.due.today=rgb400
color.due=color1
color.overdue=color9

# Report: burndown
color.burndown.done=on rgb010
color.burndown.pending=on color9
color.burndown.started=on color11

# Report: history
color.history.add=color0 on rgb500
color.history.delete=color0 on rgb550
color.history.done=color0 on rgb050

# Report: summary
color.summary.background=white on color0
color.summary.bar=black on rgb141

# Command: calendar
color.calendar.due.today=color15 on color1
color.calendar.due=color0 on color1
color.calendar.holiday=color0 on color11
color.calendar.scheduled=rgb013 on color15
color.calendar.overdue=color0 on color9
color.calendar.today=color15 on rgb013
color.calendar.weekend=on color235
color.calendar.weeknumber=rgb013

# Command: sync
color.sync.added=rgb010
color.sync.changed=color11
color.sync.rejected=color9

# Command: undo
color.undo.after=color2
color.undo.before=color1

# UDA priority
uda.priority.type=string                       # UDA priority is a string type
uda.priority.label=Priority                    # UDA priority has a display label'
uda.priority.values=H,M,L,                     # UDA priority values are 'H', 'M', 'L' or ''
                                                # UDA priority sorting is 'H' > 'M' > 'L' > '' (highest to lowest)
#uda.priority.default=M                        # UDA priority default value of 'M'
urgency.uda.priority.H.coefficient=6.0         # UDA priority coefficient for value 'H'
urgency.uda.priority.M.coefficient=3.9         # UDA priority coefficient for value 'M'
urgency.uda.priority.L.coefficient=1.8         # UDA priority coefficient for value 'L'

#default.project=foo                           # Default project for 'add' command
#default.due=eom                               # Default due date for 'add' command
#default.scheduled=eom                         # Default scheduled date for 'add' command
default.command=next                           # When no arguments are specified
default.timesheet.filter=( +PENDING and start.after:now-4wks ) or ( +COMPLETED and end.after:now-4wks )

_forcecolor=0                                  # Forces color to be on, even for non TTY output
complete.all.tags=0                            # Include old tag names in '_ags' command
list.all.projects=0                            # Include old project names in 'projects' command
summary.all.projects=0                         # Include old project names in 'summary' command
list.all.tags=0                                # Include old tag names in 'tags' command
print.empty.columns=0                          # Print columns which have no data for any task
debug=0                                        # Display diagnostics
debug.tls=0                                    # Sync diagnostics
sugar=1                                        # Syntactic sugar
obfuscate=0                                    # Obfuscate data for error reporting
fontunderline=1                                # Uses underlines rather than -------

# WARNING: Please read the documentation (man task-sync) before setting up
#          Taskwarrior for Taskserver synchronization.
#taskd.ca=<certificate file>
#taskd.certificate=<certificate file>
#taskd.credentials=<organization>/<name>/<password>
#taskd.server=<server>:<port>
taskd.trust=strict
#taskd.trust=ignore hostname
#taskd.trust=allow all
taskd.ciphers=NORMAL

# Aliases - alternate names for commands
alias.rm=delete                                # Alias for the delete command
alias.history=history.monthly                  # Prefer monthly over annual history reports
alias.ghistory=ghistory.monthly                # Prefer monthly graphical over annual history reports
alias.burndown=burndown.weekly                 # Prefer the weekly burndown chart

# Reports

report.long.description=All details of tasks
report.long.labels=ID,A,Created,Mod,Deps,P,Project,Tags,Recur,Wait,Sched,Due,Until,Description
report.long.columns=id,start.active,entry,modified.age,depends,priority,project,tags,recur,wait.remaining,scheduled,due,until,description
report.long.filter=status:pending -WAITING
report.long.sort=modified-
report.long.context=1

report.list.description=Most details of tasks
report.list.labels=ID,Active,Age,D,P,Project,Tags,R,Sch,Due,Until,Description,Urg
report.list.columns=id,start.age,entry.age,depends.indicator,priority,project,tags,recur.indicator,scheduled.countdown,due,until.remaining,description.count,urgency
report.list.filter=status:pending -WAITING
report.list.sort=start-,due+,project+,urgency-
report.list.context=1

report.ls.description=Few details of tasks
report.ls.labels=ID,A,D,Project,Tags,R,Wait,S,Due,Until,Description
report.ls.columns=id,start.active,depends.indicator,project,tags,recur.indicator,wait.remaining,scheduled.countdown,due.countdown,until.countdown,description.count
report.ls.filter=status:pending -WAITING
report.ls.sort=start-,description+
report.ls.context=1

report.minimal.description=Minimal details of tasks
report.minimal.labels=ID,Project,Tags,Description
report.minimal.columns=id,project,tags.count,description.count
report.minimal.filter=status:pending
report.minimal.sort=project+/,description+
report.minimal.context=1

report.newest.description=Newest tasks
report.newest.labels=ID,Active,Created,Age,Mod,D,P,Project,Tags,R,Wait,Sch,Due,Until,Description
report.newest.columns=id,start.age,entry,entry.age,modified.age,depends.indicator,priority,project,tags,recur.indicator,wait.remaining,scheduled.countdown,due,until.age,description
report.newest.filter=status:pending
report.newest.sort=entry-
report.newest.context=1

report.oldest.description=Oldest tasks
report.oldest.labels=ID,Active,Created,Age,Mod,D,P,Project,Tags,R,Wait,Sch,Due,Until,Description
report.oldest.columns=id,start.age,entry,entry.age,modified.age,depends.indicator,priority,project,tags,recur.indicator,wait.remaining,scheduled.countdown,due,until.age,description
report.oldest.filter=status:pending
report.oldest.sort=entry+
report.oldest.context=1

report.overdue.description=Overdue tasks
report.overdue.labels=ID,Active,Age,Deps,P,Project,Tag,R,S,Due,Until,Description,Urg
report.overdue.columns=id,start.age,entry.age,depends,priority,project,tags,recur.indicator,scheduled.countdown,due,until,description,urgency
report.overdue.filter=status:pending and +OVERDUE
report.overdue.sort=urgency-,due+
report.overdue.context=1

report.active.description=Active tasks
report.active.labels=ID,Started,Active,Age,D,P,Project,Tags,Recur,W,Sch,Due,Until,Description
report.active.columns=id,start,start.age,entry.age,depends.indicator,priority,project,tags,recur,wait,scheduled.remaining,due,until,description
report.active.filter=status:pending and +ACTIVE
report.active.sort=project+,start+
report.active.context=1

report.completed.description=Completed tasks
report.completed.labels=ID,UUID,Created,Completed,Age,Deps,P,Project,Tags,R,Due,Description
report.completed.columns=id,uuid.short,entry,end,entry.age,depends,priority,project,tags,recur.indicator,due,description
report.completed.filter=status:completed
report.completed.sort=end+
report.completed.context=1

report.recurring.description=Recurring Tasks
report.recurring.labels=ID,Active,Age,D,P,Parent,Project,Tags,Recur,Sch,Due,Until,Description,Urg
report.recurring.columns=id,start.age,entry.age,depends.indicator,priority,parent.short,project,tags,recur,scheduled.countdown,due,until.remaining,description,urgency
report.recurring.filter=(status:pending and +CHILD) or (status:recurring and +PARENT)
report.recurring.sort=due+,urgency-,entry+
report.recurring.context=1

report.waiting.description=Waiting (hidden) tasks
report.waiting.labels=ID,A,Age,D,P,Project,Tags,R,Wait,Remaining,Sched,Due,Until,Description
report.waiting.columns=id,start.active,entry.age,depends.indicator,priority,project,tags,recur.indicator,wait,wait.remaining,scheduled,due,until,description
report.waiting.filter=+WAITING
report.waiting.sort=due+,wait+,entry+
report.waiting.context=1

report.all.description=All tasks
report.all.labels=ID,St,UUID,A,Age,Done,D,P,Project,Tags,R,Wait,Sch,Due,Until,Description
report.all.columns=id,status.short,uuid.short,start.active,entry.age,end.age,depends.indicator,priority,project.parent,tags.count,recur.indicator,wait.remaining,scheduled.remaining,due,until.remaining,description
report.all.sort=entry-
report.all.context=1

report.next.description=Most urgent tasks
report.next.labels=ID,Active,Age,Deps,P,Project,Tag,Recur,S,Due,Until,Description,Urg
report.next.columns=id,start.age,entry.age,depends,priority,project,tags,recur,scheduled.countdown,due.relative,until.remaining,description,urgency
report.next.filter=status:pending -WAITING limit:page
report.next.sort=urgency-
report.next.context=1

report.ready.description=Most urgent actionable tasks
report.ready.labels=ID,Active,Age,D,P,Project,Tags,R,S,Due,Until,Description,Urg
report.ready.columns=id,start.age,entry.age,depends.indicator,priority,project,tags,recur.indicator,scheduled.countdown,due.countdown,until.remaining,description,urgency
report.ready.filter=+READY
report.ready.sort=start-,urgency-
report.ready.context=1

report.blocked.description=Blocked tasks
report.blocked.columns=id,depends,project,priority,due,start.active,entry.age,description
report.blocked.labels=ID,Deps,Proj,Pri,Due,Active,Age,Description
report.blocked.sort=due+,priority-,start-,project+
report.blocked.filter=status:pending -WAITING +BLOCKED
report.blocked.context=1

report.unblocked.description=Unblocked tasks
report.unblocked.columns=id,depends,project,priority,due,start.active,entry.age,description
report.unblocked.labels=ID,Deps,Proj,Pri,Due,Active,Age,Description
report.unblocked.sort=due+,priority-,start-,project+
report.unblocked.filter=status:pending -WAITING -BLOCKED
report.unblocked.context=1

report.blocking.description=Blocking tasks
report.blocking.labels=ID,UUID,A,Deps,Project,Tags,R,W,Sch,Due,Until,Description,Urg
report.blocking.columns=id,uuid.short,start.active,depends,project,tags,recur,wait,scheduled.remaining,due.relative,until.remaining,description.count,urgency
report.blocking.sort=urgency-,due+,entry+
report.blocking.filter=status:pending -WAITING +BLOCKING
report.blocking.context=1

report.timesheet.filter=(+PENDING and start.after:now-4wks) or (+COMPLETED and end.after:now-4wks)
report.timesheet.context=0
";