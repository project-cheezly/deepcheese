import * as d3 from 'd3';

const locale = d3.timeFormatLocale({
    dateTime: "%x, %X",
    date: "%-m/%-d/%Y",
    time: "%-I:%M:%S %p",
    periods: ["오전", "오후"],
    days: ["일요일", "월요일", "화요일", "수요일", "목요일", "금요일", "토요일"],
    shortDays: ["일", "월", "화", "수", "목", "금", "토"],
    months: ["1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월", "12월"],
    shortMonths: ["1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월", "12월"]
});

const formatMillisecond = locale.utcFormat(".%L"),
    formatSecond = locale.utcFormat(":%S"),
    formatMinute = locale.utcFormat("%I:%M"),
    formatHour = locale.utcFormat("-%H시"),
    formatDay = locale.utcFormat("%-d일"),
    formatWeek = locale.utcFormat("%b %d"),
    formatMonth = locale.utcFormat("%B"),
    formatYear = locale.utcFormat("%Y");

const localeFormatMillisecond = d3.timeFormat(".%L"),
    localeFormatSecond = d3.timeFormat(":%S"),
    localeFormatMinute = d3.timeFormat("%I:%M"),
    localeFormatHour = d3.timeFormat("%-H시"),
    localeFormatDay = d3.timeFormat("%-d일"),
    localeFormatWeek = d3.timeFormat("%b %d"),
    localeFormatMonth = d3.timeFormat("%B"),
    localeFormatYear = d3.timeFormat("%Y");


function multiFormat(date) {
    return (d3.utcSecond(date) < date ? formatMillisecond
        : d3.utcMinute(date) < date ? formatSecond
        : d3.utcHour(date) < date ? formatMinute
        : d3.utcDay(date) < date ? formatHour
        : d3.utcMonth(date) < date ? (d3.utcWeek(date) < date ? formatDay : formatWeek)
        : d3.utcYear(date) < date ? formatMonth
        : formatYear)(date);
}

function localeMultiFormat(date) {
    return (d3.timeSecond(date) < date ? localeFormatMillisecond
        : d3.timeMinute(date) < date ? localeFormatSecond
        : d3.timeHour(date) < date ? localeFormatMinute
        : d3.timeDay(date) < date ? localeFormatHour
        : d3.timeMonth(date) < date ? (d3.timeWeek(date) < date ? localeFormatDay : localeFormatWeek)
        : d3.timeYear(date) < date ? localeFormatMonth
        : localeFormatYear)(date);
}

export { multiFormat, localeMultiFormat };