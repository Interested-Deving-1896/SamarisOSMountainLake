export function MiniChart(props: { values: number[]; stroke: string }) {
  const width = 260;
  const height = 82;
  const maxValue = Math.max(1, ...props.values, 100);
  const points = props.values
    .map((value, index) => {
      const x = props.values.length <= 1 ? 0 : (index / Math.max(1, props.values.length - 1)) * width;
      const y = height - (value / maxValue) * (height - 8) - 4;
      return `${x},${y}`;
    })
    .join(" ");
  return (
    <svg className="utilities__chart" viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="none">
      <polyline
        fill="none"
        stroke={props.stroke}
        strokeWidth="2.5"
        strokeLinejoin="round"
        strokeLinecap="round"
        points={points || `0,${height / 2} ${width},${height / 2}`}
      />
    </svg>
  );
}
