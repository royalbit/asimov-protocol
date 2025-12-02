# Freshness Protocol - Date-Aware Search (Priority 1)
# Hardcoded in binary. Cannot be bypassed.
# Date: {TODAY}

context:
  today: {TODAY}
  year: {YEAR}
  reminder: You have a training cutoff. Today is {TODAY}.

triggers:
  always_search:
    - version numbers (library versions, API versions)
    - pricing (cloud costs, SaaS pricing)
    - API documentation (endpoints, parameters)
    - current/latest (anything time-sensitive)
    - release notes and changelogs
    - {YEAR} events and updates

action: For time-sensitive topics, SEARCH FIRST. Do not rely on training data.
