import matplotlib.pyplot as plt
import pandas as pd
import os
import numpy as np
from scipy.stats import norm, pearsonr

plt.rcParams['savefig.dpi'] = 500
plt.rcParams.update({'font.size': 13})
plt.rcParams["font.family"] = "sans-serif"

for file in os.listdir("."):
    if ".csv" in file:
        data = pd.read_csv(file, header=0)

        nmbr_markets = 0
        for col in data.columns:
            if "price" in col:
                nmbr_markets += 1

        if nmbr_markets == 1:
            alphas = [1]
        else:
            alphas = np.linspace(0.5, 1, nmbr_markets)

        # CDF
        plt.figure()
        for col in data.columns:
            if "price" in col:
                m_name = col.split("[")[1].split("]")[0]

                log_rets = (data[col].shift(1) / data[col]).dropna().apply(np.log)
                norm_log_rets = ((log_rets - log_rets.mean()) / log_rets.std())

                count = len(norm_log_rets)
                x = np.linspace(1, 0, count)

                # norm_cdf = norm.cdf(norm_log_rets)
                samples = np.abs(np.array(norm_log_rets))
                samples.sort()

                plt.scatter(samples, x, s=0.5, label="Market {}".format(m_name), c="C{}".format(m_name))

        samples_n = np.abs(norm.rvs(size=count))
        samples_n.sort()
        
        plt.plot(samples_n, x, linewidth=0.7, c='k', label="N(0,1)")
        plt.yscale('log')
        plt.xscale('log')
        plt.ylim([10e-5, 10e-1])
        plt.xlim([0.1,10])
        plt.ylabel("P>(|Ret|)")
        plt.xlabel("|Ret|")
        plt.legend(bbox_to_anchor=(0, 1, 1, 0), loc="lower left",
                    mode="expand", ncol=nmbr_markets, markerscale=6)
        # plt.grid()
        plt.savefig("plots/{}_cdf.pdf".format(file.split(".")[0]), bbox_inches='tight')

        # Returns and Volatility
        plt.figure()
        ax1 = plt.subplot(211)
        ax2 = plt.subplot(212, sharex=ax1)
        for col in data.columns:
            if "price" in col:
                m_name = col.split("[")[1].split("]")[0]
                # log_rets = (data[col].shift(1) / data[col]).dropna().apply(np.log)
                price = data[col]

                # ax1.plot(log_rets, label="Market {}".format(
                #     m_name), alpha=alphas[-int(m_name)], linewidth=0.7, c="C{}".format(m_name))
                ax1.plot(price, label="Market {}".format(
                    m_name), alpha=alphas[-int(m_name)], linewidth=0.7, c="C{}".format(m_name))
            if "volatility" in col:
                m_name = col.split("[")[1].split("]")[0]
                ax2.plot(data[col], label="Market {}".format(
                    m_name), alpha=alphas[-int(m_name)], linewidth=0.7, c="C{}".format(m_name))

        ax1.set_ylabel("Price")
        # ax1.set_ylabel("Return")
        ax1.grid()

        ax2.set_ylabel("Volatility")
        ax2.grid()
        plt.xlabel("Time step")
        if nmbr_markets > 1:
            ax1.legend(bbox_to_anchor=(0, 1, 1, 0), loc="lower left",
                       mode="expand", ncol=nmbr_markets)
        plt.savefig("plots/{}_returns_plus_vol.pdf".format(file.split(".")[0]), bbox_inches='tight')


        # Median wealth
        plt.figure()
        plt.plot(data["median_wealth"], linewidth=0.7)

        plt.ylabel("Median wealth")
        plt.xlabel("Time step")
        plt.grid()
        plt.savefig("plots/{}_median_wealth.pdf".format(file.split(".")[0]), bbox_inches='tight')

        # Autocorrelation
        plt.figure()
        for col in data.columns:
            if "price" in col:
                m_name = col.split("[")[1].split("]")[0]

                rets = (data[col].shift(1) / data[col]).dropna()

                abs_rets = np.abs((data[col].shift(1) - data[col]).dropna())
                # print(rets, abs_rets)

                tau = np.linspace(0, 100, 101, dtype=int)
                ac_rets = [pearsonr(np.array(rets.shift(t)[t:]), np.array(rets[t:]))[0] for t in tau]
                ac_abs_rets = [pearsonr(np.array(abs_rets.shift(t)[t:]), np.array(abs_rets[t:]))[0] for t in tau]
                noise_lvl = 3 / np.sqrt(len(abs_rets))

                plt.plot(tau, ac_rets, linewidth=0.7, c="C{}".format(m_name))

                plt.plot(tau, ac_abs_rets, linestyle=(0, (1, 10)), linewidth=1, c="C{}".format(int(m_name)))
                plt.scatter(tau, ac_abs_rets, s=1.7, label="Market {}".format(m_name), c="C{}".format(int(m_name)))

        plt.axhline(noise_lvl, linestyle="--", c='k')
        plt.axhline(-noise_lvl, linestyle="--", c='k')

        plt.xlabel(r'$\tau$')
        plt.ylabel(r'$C(\tau)$')
        if nmbr_markets > 0:
            plt.legend(bbox_to_anchor=(0, 1, 1, 0), loc="lower left",
                        mode="expand", ncol=nmbr_markets, markerscale=6)

        plt.savefig("plots/{}_autocorrelation.pdf".format(file.split(".")[0]), bbox_inches='tight')


        # plt.figure()
        # for col in data.columns:
        #     if "agent_assets" in col:
        #         agent_name = col.split("[")[1].split("]")[0]
        #         plt.plot(data[col], label="Agent {}".format(
        #             m_name), alpha=alphas[-int(m_name)])

        # plt.ylabel("Volatility")
        # plt.xlabel("Time step")
        # if nmbr_markets > 1:
        #     plt.legend(bbox_to_anchor=(0, 1, 1, 0), loc="lower left",
        #                mode="expand", ncol=nmbr_markets)
        # plt.grid()
        # plt.savefig("plots/{}_volatility.pdf".format(file.split(".")[0]))

        # plt.figure()
        # for col in data.columns:
        #     if "price" in col:
        #         m_name = col.split("[")[1].split("]")[0]
        #         log_rets = data[col].apply(np.log10).dropna() - data[col].shift(1).apply(
        #             np.log10).dropna()

        #         plt.plot(log_rets, label="Market {}".format(
        #             m_name), alpha=alphas[-int(m_name)])

        # plt.ylabel("Return")
        # plt.xlabel("Time step")
        # if nmbr_markets > 1:
        #     plt.legend(bbox_to_anchor=(0, 1, 1, 0), loc="lower left",
        #                mode="expand", ncol=nmbr_markets)
        # plt.grid()
        # plt.savefig("plots/{}_returns.pdf".format(file.split(".")[0]))

        # plt.figure()
        # for col in data.columns:
        #     if "volatility" in col:
        #         m_name = col.split("[")[1].split("]")[0]
        #         plt.plot(data[col], label="Market {}".format(
        #             m_name), alpha=alphas[-int(m_name)])

        # plt.ylabel("Volatility")
        # plt.xlabel("Time step")
        # if nmbr_markets > 1:
        #     plt.legend(bbox_to_anchor=(0, 1, 1, 0), loc="lower left",
        #                mode="expand", ncol=nmbr_markets)
        # plt.grid()
        # plt.savefig("plots/{}_volatility.pdf".format(file.split(".")[0]))
