import { useEffect, useState } from "react";
import { ApexOptions } from "apexcharts";
import Chart from "react-apexcharts";
import { useTheme } from "../admin/theme-provider";

interface ThemeAwareChartProps {
  options: ApexOptions;
  series: ApexAxisChartSeries | ApexNonAxisChartSeries;
  type:
    | "line"
    | "bar"
    | "area"
    | "pie"
    | "donut"
    | "radialBar"
    | "scatter"
    | "bubble"
    | "heatmap"
    | "candlestick"
    | "boxPlot"
    | "radar"
    | "polarArea"
    | "rangeBar"
    | "treemap";
  height?: number | string;
}

const ThemeChart = ({
  options,
  series,
  type,
  height,
}: ThemeAwareChartProps) => {
  const { theme } = useTheme();
  const [chartOptions, setChartOptions] = useState<ApexOptions>(options);

  useEffect(() => {
    const newOptions: ApexOptions = {
      ...options,
      chart: {
        ...options.chart,
        foreColor: theme === "dark" ? "#FFFFFF" : "#000000",
      },
      xaxis: {
        ...options.xaxis,
        labels: {
          style: {
            colors: theme === "dark" ? "#FFFFFF" : "#000000",
          },
        },
      },
      yaxis: {
        ...options.yaxis,
        labels: {
          style: {
            colors: theme === "dark" ? "#FFFFFF" : "#000000",
          },
        },
      },
      tooltip: {
        theme: theme === "dark" ? "dark" : "light",
        style: {
          fontSize: "12px",
          fontFamily: "Arial, sans-serif",
        },
        y: {
          formatter: (val: number) => `$${val.toFixed(2)}`,
        },
      },
    };
    setChartOptions(newOptions);
  }, [theme, options]);

  return (
    <Chart options={chartOptions} series={series} type={type} height={height} />
  );
};

export default ThemeChart;
