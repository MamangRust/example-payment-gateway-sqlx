import ChartMonthAmount from "./ChartMonthAmount";
import ChartYearAmount from "./ChartYearAmount";

interface ChartDashboardSaldoProps {
    loadingMonthBalance: boolean;
    loadingYearBalance: boolean;
    monthlyBalances: any;
    yearlyBalances: any;
}

export default function ChartDashboarSaldo({
    loadingMonthBalance,
    loadingYearBalance,
    monthlyBalances,
    yearlyBalances,
}: ChartDashboardSaldoProps) {

    return (
        <>
            <ChartMonthAmount loadingMonthBalance={loadingMonthBalance} monthlyBalances={monthlyBalances} />
            <ChartYearAmount loadingYearBalance={loadingYearBalance} yearlyBalances={yearlyBalances} />
        </>
    )
}