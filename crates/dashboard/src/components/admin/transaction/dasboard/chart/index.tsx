import ChartMonthTransactionAmount from "./ChartMonthAmount";
import ChartMonthTransactionPaymentMethod from "./ChartMonthPaymentMethod";
import ChartYearTransactionAmount from "./ChartYearAmount";
import ChartYearTransactionPaymentMethod from "./ChartYearPaymentMethod";

interface ChartDashboardTransactionProps {
    loadingMonthTransactionAmount: boolean;
    loadingMonthTransactionMethod: boolean;
    loadingYearTransactionAmount: boolean;
    loadingYearTransactionMethod: boolean

    monthlyAmount: any;
    monthlyPaymentMethod: any;
    yearlyAmount: any;
    yearlyPaymentMethod: any;
}

export default function ChartDashboarTransaction({
    loadingMonthTransactionAmount,
    loadingMonthTransactionMethod,
    loadingYearTransactionAmount,
    loadingYearTransactionMethod,
    monthlyAmount,
    monthlyPaymentMethod,
    yearlyAmount,
    yearlyPaymentMethod
}: ChartDashboardTransactionProps) {

    return (
        <>
            <ChartMonthTransactionPaymentMethod loadingMonthTransactionMethod={loadingMonthTransactionMethod} monthlyPaymentMethod={monthlyPaymentMethod} />
            <ChartYearTransactionPaymentMethod loadingYearTransactionMethod={loadingYearTransactionMethod} yearlyPaymentMethod={yearlyPaymentMethod} />
            <ChartMonthTransactionAmount loadingMonthTransactionAmount={loadingMonthTransactionAmount} monthlyAmount={monthlyAmount} />
            <ChartYearTransactionAmount loadingYearTransactionAmount={loadingYearTransactionAmount} yearlyAmount={yearlyAmount} />
        </>
    )
}