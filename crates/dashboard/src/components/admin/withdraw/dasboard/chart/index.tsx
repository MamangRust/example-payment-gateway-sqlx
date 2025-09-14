import ChartMonthWithdrawAmount from "./ChartMonthAmount";
import ChartYearWithdrawAmount from "./ChartYearAmount";

interface ChartDashboardWithdrawProps {
    loadingMonthWithdrawAmount: boolean;
    loadingYearWithdrawAmount: boolean;

    monthlyAmount: any;
    yearlyAmount: any;
}

export default function ChartDashboarWithdraw({
    loadingMonthWithdrawAmount,
    loadingYearWithdrawAmount,
    monthlyAmount,
    yearlyAmount,
}: ChartDashboardWithdrawProps) {

    return (
        <>
            <ChartMonthWithdrawAmount loadingMonthWithdrawAmount={loadingMonthWithdrawAmount} monthlyAmount={monthlyAmount} />
            <ChartYearWithdrawAmount loadingYearWithdrawAmount={loadingYearWithdrawAmount} yearlyAmount={yearlyAmount} />
        </>
    )
}