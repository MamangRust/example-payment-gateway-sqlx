import ChartMonthTransferAmount from "./ChartMonthAmount";
import ChartYearTransferAmount from "./ChartYearAmount";

interface ChartDashboardTransferProps {
    loadingMonthTransferAmount: boolean;
    loadingYearTransferAmount: boolean;
    monthlyAmount: any;
    yearlyAmount: any;
}

export default function ChartDashboarTransfer({
    loadingMonthTransferAmount,
    loadingYearTransferAmount,
    monthlyAmount,
    yearlyAmount,
}: ChartDashboardTransferProps) {

    return (
        <>
            <ChartMonthTransferAmount loadingMonthTransferAmount={loadingMonthTransferAmount} monthlyAmount={monthlyAmount} />
            <ChartYearTransferAmount loadingYearTransferAmount={loadingYearTransferAmount} yearlyAmount={yearlyAmount} />
        </>
    )
}