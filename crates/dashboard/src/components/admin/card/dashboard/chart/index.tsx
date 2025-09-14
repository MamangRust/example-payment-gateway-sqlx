import CharthMonthBalances from "./ChartMonthBalances";
import ChartYearBalances from "./ChartYearBalances";
import ChartMonthWithdrawAmount from "./ChartMonthWithdrawAmount";
import ChartYearWithdrawAmount from "./CharyYearWithdrawAmount";
import ChartMonthTopupAmount from "./ChartMonthTopupAmount";
import ChartYearTopupAmount from "./CharyYearTopupAmount";
import ChartMonthTransaction from "./ChartMonthTransaction";
import ChartYearTransaction from "./ChartYearTransaction";
import ChartMonthTransferSender from "./ChartMonthTransferSender";
import ChartYearTransferSender from "./ChartYearTransferSender";
import ChartMonthTransferReceiver from "./ChartMonthTransferReceiver";
import ChartYearTransferReceiver from "./ChartYearTransferReceiver";


interface ChartDashboardCardNumberProps {
    loadingMonthBalance: boolean;
    loadingYearBalance: boolean;
    loadingMonthWithdrawAmount: boolean;
    loadingYearWithdrawAmount: boolean;
    loadingMonthTopupAmount: boolean;
    loadingYearTopupAmount: boolean;
    loadingMonthTransaction: boolean;
    loadingYearTransaction: boolean;
    loadingMonthTransferSender: boolean;
    loadingYearTransferSender: boolean;
    loadingMonthTransferReceiver: boolean;
    loadingYearTransferReceiver: boolean;

    monthlyBalances: any;
    yearlyBalances: any;
    monthlyWithdraws: any;
    yearlyWithdraws: any;
    monthlyTopups: any;
    yearlyTopups: any;
    monthlyTransactions: any;
    yearlyTransactions: any;
    monthlyTransfersSender: any;
    yearlyTransfersSender: any;
    monthlyTransfersReceiver: any;
    yearlyTransfersReceiver: any;
}



export default function ChartDashboardCard({ loadingMonthBalance, monthlyBalances, loadingYearBalance, yearlyBalances, loadingMonthWithdrawAmount, monthlyWithdraws, loadingYearWithdrawAmount, yearlyWithdraws, loadingMonthTopupAmount, monthlyTopups, loadingYearTopupAmount, yearlyTopups, loadingMonthTransaction, monthlyTransactions, loadingYearTransaction, yearlyTransactions, loadingMonthTransferSender, monthlyTransfersSender, loadingYearTransferSender, yearlyTransfersSender, loadingMonthTransferReceiver, monthlyTransfersReceiver, loadingYearTransferReceiver, yearlyTransfersReceiver
}: ChartDashboardCardNumberProps) {
    return (
        <>
            <CharthMonthBalances
                loadingMonthBalance={loadingMonthBalance}
                monthlyBalances={monthlyBalances}
            />
            <ChartYearBalances
                loadingYearBalance={loadingYearBalance}
                yearlyBalances={yearlyBalances}
            />

            <ChartMonthWithdrawAmount
                loadingMonthWithdrawAmount={loadingMonthWithdrawAmount}
                monthlyWithdraws={monthlyWithdraws}
            />
            <ChartYearWithdrawAmount
                loadingYearWithdrawAmount={loadingYearWithdrawAmount}
                yearlyWithdraws={yearlyWithdraws}
            />

            <ChartMonthTopupAmount
                loadingMonthTopupAmount={loadingMonthTopupAmount}
                monthlyTopups={monthlyTopups}
            />
            <ChartYearTopupAmount
                loadingYearTopupAmount={loadingYearTopupAmount}
                yearlyTopups={yearlyTopups}
            />

            <ChartMonthTransaction
                loadingMonthTransaction={loadingMonthTransaction}
                monthlyTransactions={monthlyTransactions}
            />
            <ChartYearTransaction
                loadingYearTransaction={loadingYearTransaction}
                yearlyTransactions={yearlyTransactions}
            />

            <ChartMonthTransferSender
                loadingMonthTransferSender={loadingMonthTransferSender}
                monthlyTransfersSender={monthlyTransfersSender}
            />
            <ChartYearTransferSender
                loadingYearTransferSender={loadingYearTransferSender}
                yearlyTransfersSender={yearlyTransfersSender}
            />
            <ChartMonthTransferReceiver
                loadingMonthTransferReceiver={loadingMonthTransferReceiver}
                monthlyTransfersReceiver={monthlyTransfersReceiver}
            />
            <ChartYearTransferReceiver
                loadingYearTransferReceiver={loadingYearTransferReceiver}
                yearlyTransfersReceiver={yearlyTransfersReceiver}
            />
        </>
    )

}