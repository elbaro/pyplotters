<table>
<tbody>
	<tr><th></th><th></th><th colspan='7'>Chart Axis Dtype</th></tr>
 <tr><td>&nbsp;</td><td>&nbsp;</td><td colspan='4'>Numeric types</td><td colspan='4'>Naive chrono types</td></tr>
 <tr><td colspan='2'>Python Series Type</td><td>i32</td><td>i64</td><td>f32</td><td>f64</td><td>DateTime</td><td>Date</td><td>Time</td></tr>
 <tr><td rowspan='4'>Numeric types</td><td>List[int]</td><td>o</td><td>o</td><td>o</td><td>o</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td></tr>
 <tr><td>np.array(dtype=i32)</td><td>o</td><td>o</td><td>o</td><td>o</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td></tr>
 <tr><td>List[float]</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td></tr>
 <tr><td>np.array(dtype=float)</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td></tr>
 <tr><td rowspan='8'>Naive chrono types</td><td>List[datetime.datetime]</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>o</td></tr>
 <tr><td>List[datetime.date]</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>&nbsp;</td></tr>
 <tr><td>List[datetime.time]</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td></tr>
 <tr><td>ezel.StringDateTime</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td></tr>
 <tr><td>ezel.Timestamp</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>o</td></tr>
 <tr><td>ezel.MilliTimestamp</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>o</td></tr>
 <tr><td>ezel.MicroTimestamp</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>o</td></tr>
 <tr><td>ezel.NanoTimestamp</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td><td>o</td><td>o</td><td>o</td></tr>
</tbody></table>
