import { formatRupiah } from "@/helpers/formatRupiah";
import TotalSaldoCard from "./TotalSaldo";
import { DashboardCard } from "@/types/model";
import TotalWithdrawCard from "./TotalWithdraw";
import TotalTransactionCard from "./TotalTransaction";
import TotalTopupCard from "./TotalTopup";
import TotalTransfer from "./TotalTransfer";


export default function TotalCardDashboard({ dashboardCardNumber }: { dashboardCardNumber: DashboardCard }) {
    return (
        <>
            <TotalSaldoCard total_balance={formatRupiah(dashboardCardNumber?.total_balance ?? 0)} />
            <TotalWithdrawCard total_withdraw={formatRupiah(dashboardCardNumber?.total_withdraw ?? 0)} />
            <TotalTransactionCard total_transaction={formatRupiah(dashboardCardNumber?.total_transaction ?? 0)} />
            <TotalTopupCard total_topup={formatRupiah(dashboardCardNumber?.total_topup ?? 0)} />
            <TotalTransfer total_transfer={formatRupiah(dashboardCardNumber?.total_transfer ?? 0)} />
        </>
    )
}