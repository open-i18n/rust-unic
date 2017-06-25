# UNIC Versioning

UNIC follows *Semantic Versioning* practices for each component. In addition, when a component
is updated, its parent components will also receive a version update, with the same version
number.

In other words, when component `unic-aaa-bbb` is updated to version `x.y.z`, so are `unic-aaa`
and `unic`. And, when components `unic-aaa-bbb` and `unic-ddd-eee` are update together,
`unic-aaa-bbb`, `unic-aaa`, `unic-ddd-eee`, `unic-ddd`, and `unic` will receive version updates,
all with the same version number `x.y.z`.

## Unicode Versions

Because of how Character Properties are implemented in `unic-ucd` components, upgrading Unicode
data sources to a newer (major or minor, but not micro) update of Unicode results in an
API-breaking change, therefore a breaking version update.

This, also, allows applications to *intentionally* opt-in for Unicode version upgrades, which
can be very important for some applications, like search indices, as there as Unicode text
processing algorithms that are not promissed to be stable between Unicode versions. See [Unicode
Character Encoding Stability Policies](http://unicode.org/policies/stability_policy.html) for
more details.

As of 2017, Unicode is expected to have a major update every year, with no minor updates.
Therefore, UNIC will have at least one API-breaking version update each year, even if all major
parts of the API are stable.
