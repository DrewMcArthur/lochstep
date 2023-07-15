# worker.coop

the app for managing a business owned and run by its workers.

## background

the current paradigm has a split between the owners and workers of a business - the employer and the employee. worker-owned cooperatives exist, as well as ESOP-based companies, however both fall short.

a coop does not account for differences between levels of effort between workers, and cannot scale as a result. ESOPs are better than publicly-traded corporations, but fall into the same pitfalls of coprorate hierarchy.

the goal of this app is to support a new type of company, one where ownership is directly linked to labor, and where each worker has a voice in collective decisions that affect them.

## proposals and voting

in order to run a business without concentrated ownership, decisions must be made collectively via proposals and votes. these can be specific or general depending on the company; one business might vote on seemingly minor decisions, and another might reserve voting only for ousting a bad manager or employee.

the goal of collective decision making is to allow for (but not require) any worker to have a say in decisions that affect them.
to meet these goals, proposals can be discussed with reddit-style comments, and voted on in order to enact the proposal's effects (e.g. a raise for a position)

when a worker votes on a proposal, two votes are recorded: a one-person-one-vote count, as well as the number of voting shares they have allocated. this allows companies and proposals to flexibly balance how they weigh and value their decision making, whether they'd like equal voice and participation, or a weighted vote according to stake in the business.

voting should also implement _liquid democracy_, where a worker can allocate a delegate. if that worker does not vote on a given proposal, their voting shares are allocated to their delegate's vote/decision. for transparency, only public votes can have shares delegated to them. if you wish to have shares delegated to you, your votes must be public, and if you wish to vote privately, you cannot vote on others' behalf.

proposals can be implemented with a smart-contract-like behavior, directly affecting variables at the core of the company's operations, such as the modifiers in the share issuance equation, or the wage for a certain position, or the rate of accrual for PTO or sick time.

## economics of ownership shares

Stephanie Kelton's Modern Monetary Theory discusses economics of monetary systems from the perspective of the money issuer. It is from this perspective that we design share issuance, where the company creates shares when they are issued, and destroys them upon repurchase.

shares are automatically created and distributed with each unit of labor by a worker, according to the following formula

```
shares = (time * T') * (wage * W')
```

where `T'` and `W'` are modifiers, and variable per company. These modifiers can adjust how the company values time spent at work and wage as a proxy for level of employee. For example, a manager earning $20/hr might earn 33% more shares per hour worked compared to a cashier earning $15/hr.

shares are used as a proxy for ownership, and used to determine weights when voting on decisions, and profit sharing allocation.

```
share price = total company treasury / total shares issued
```

at any time, any worker can redeem their shares for their share of the company treasury. shares can also be bought using a similar formula, which keeps the share price stable by issuing new shares in concert with the increase in the treasury's monetary value.

```
shares received = share price * money paid
```

## this app

this app will utilize a rust backend and an htmx frontend. initial state will be stored in a sqlite db, and proposals will likely utilize a holochain backend once development stabilizes enough.
