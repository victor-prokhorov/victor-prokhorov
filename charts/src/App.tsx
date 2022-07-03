import React, { ReactNode } from 'react';
import './App.css';
import Highcharts from 'highcharts';
import HighchartsReact from 'highcharts-react-official';
import { ReactComponent as TrendArrowUp } from './assets/trend-arrow-up.svg';
import { ReactComponent as TrendArrowDown } from './assets/trend-arrow-down.svg';

import Card from './components/Card';

const k = {};
// const k = {};

const dataAverageTicketSize = [
  59.52,
  0.17,
  [
    ['2021/9', 59.56],
    ['2021/10', 59.7],
    ['2021/11', 59.91],
    ['2021/12', 59.38],
    ['2021/13', 59.14],
    ['2021/14', 59.37],
    ['2021/15', 59.51],
    ['2021/16', 58.08],
    ['2021/17', 60.78],
    ['2021/18', 59.98],
    ['2021/19', 60.92],
    ['2021/20', 59.49],
    ['2021/21', 59.82],
    ['2021/22', 62.18],
  ],
  [
    ['2022/9', 57.39],
    ['2022/10', 60.04],
    ['2022/11', 60.03],
    ['2022/12', 57.48],
    ['2022/13', 60.11],
    ['2022/14', 59.47],
    ['2022/15', 58.88],
    ['2022/16', 61.05],
    ['2022/17', 60.45],
    ['2022/18', 59.89],
    ['2022/19', 59.05],
    ['2022/20', 58.79],
    ['2022/21', 58.55],
    ['2022/22', 58.5],
  ],
];

const averageTicket: Highcharts.Options = {
  chart: {
    marginLeft: 0,
    marginRight: 0,
    marginTop: 0,
    marginBottom: 0,
    spacingLeft: 0,
    spacingRight: 0,
  },

  credits: {
    enabled: false,
  },

  exporting: {
    enabled: false,
  },

  subtitle: {
    text: '',
  },

  title: {
    text: '',
  },

  plotOptions: {
    line: {
      dataLabels: {
        enabled: false,
      },
    },

    series: {
      marker: {
        enabled: false,
      },
    },
  },

  yAxis: {
    labels: {
      enabled: false,
    },

    title: {
      text: '',
    },

    lineWidth: 0,

    gridLineWidth: 0,

    tickWidth: 0,

    minPadding: 0,

    maxPadding: 0,
  },

  xAxis: {
    labels: {
      enabled: false,

      staggerLines: 0,
    },

    lineWidth: 0,

    gridLineWidth: 0,

    tickWidth: 0,

    minPadding: 0,

    maxPadding: 0,
  },

  legend: {
    enabled: false,
  },

  series: [
    {
      name: 'past',

      type: 'area',

      color: '#006d44',

      opacity: 0.2,

      data: [
        ['2022/9', 57.39],
        ['2022/10', 60.04],
        ['2022/11', 60.03],
        ['2022/12', 57.48],
        ['2022/13', 60.11],
        ['2022/14', 59.47],
        ['2022/15', 58.88],
        ['2022/16', 61.05],
        ['2022/17', 60.45],
        ['2022/18', 59.89],
        ['2022/19', 59.05],
        ['2022/20', 58.79],
        ['2022/21', 58.55],
        ['2022/22', 58.5],
      ],
    },
    {
      name: 'present',
      color: '#006d44',
      type: 'line',
      data: [
        ['2022/9', 57.39],
        ['2022/10', 60.04],
        ['2022/11', 60.03],
        ['2022/12', 57.48],
        ['2022/13', 60.11],
        ['2022/14', 59.47],
        ['2022/15', 58.88],
        ['2022/16', 61.05],
        ['2022/17', 60.45],
        ['2022/18', 59.89],
        ['2022/19', 59.05],
        ['2022/20', 58.79],
        ['2022/21', 58.55],
        ['2022/22', 58.5],
      ],
      // data: this.getData(this.dataAverageTicketSize[3]),
    },
  ],
};

interface CardContainerProps {
  children?: ReactNode;
}

function CardContainer({ children }: CardContainerProps) {
  return <div className="card-container">{children}</div>;
}

interface CardProps {
  title: string;
  value: string;
  previousValue: string;
  trend: 'up' | 'down';
}

function App() {
  return (
    <div className="App">
      <CardContainer>
      <Card />
      </CardContainer>
    </div>
  );
}

export default App;
