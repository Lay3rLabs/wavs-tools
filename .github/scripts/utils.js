
function countFail(report) {
    if (!report || !report.stats || !report.results) return 0;

    // if we have any failures in stats, just count those
    if(report.stats.failures) {
        return(report.stats.failures);
    } else {
        // otherwise, it may be in some deeply nested object, before/after hooks, etc.
        return(countFailInner(report.results));
    }
}

// recursively count 'fail' property in nested objects
function countFailInner(obj) {
  let count = 0;

  if (obj === null || typeof obj !== 'object') return 0;

  if (obj.fail === true) count += 1;

  for (const key in obj) {
    if (Object.prototype.hasOwnProperty.call(obj, key)) {
      count += countFailInner(obj[key]);
    }
  }

  return count;
}

module.exports = {
    countFail,
};