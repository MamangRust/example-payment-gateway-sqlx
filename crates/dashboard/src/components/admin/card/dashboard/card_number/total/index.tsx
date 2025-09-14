import { formatRupiah } from "@/helpers/formatRupiah";
import TotalSaldoCard from "./TotalSaldo";
import {  DashboardCardCardNumber } from "@/types/model";
import TotalWithdrawCard from "./TotalWithdraw";
import TotalTransactionCard from "./TotalTransaction";
import TotalTopupCard from "./TotalTopup";
import TotalTransferSender from "./TotalTransferSender";
import TotalTransferReceiver from "./TotalTransferReceiver";


export default function TotalCardDashboardByCardNumber({dashboardCardNumber}: {dashboardCardNumber: DashboardCardCardNumber}) {
    return (
        <>
            <TotalSaldoCard total_balance={formatRupiah(dashboardCardNumber?.total_balance ?? 0)} />
            <TotalWithdrawCard total_withdraw={formatRupiah(dashboardCardNumber?.total_withdraw ?? 0)} />
            <TotalTransactionCard total_transaction= {formatRupiah(dashboardCardNumber?.total_transaction ?? 0)} />
            <TotalTopupCard total_topup= {formatRupiah(dashboardCardNumber?.total_topup ?? 0)} />
            <TotalTransferSender total_sender={formatRupiah(dashboardCardNumber?.total_transfer_send ?? 0)} />
            <TotalTransferReceiver total_receiver= {formatRupiah(dashboardCardNumber?.total_transfer_receiver ?? 0,)}/>
        </>
    )
}