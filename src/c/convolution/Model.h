#pragma once
#include <iostream>
#include <vector>
#include <fstream>
#include <assert.h>
#include "FNet.h"
#include "CLayer.h"
#include "PoolingLayer.h"
using namespace std;

class Model {
private:
	FNet FCNet;
	vector<vector<CLayer>> CNet;
	vector<vector<vector<unsigned>>> CNetDim;
	vector<PoolingLayer> PNet;
	vector<vector<unsigned>> PNetDim;
	vector<unsigned> inputSize;
public:
	//Model(vector<vector<unsigned>> &convInput, vector<unsigned> &poolSize, vector<unsigned> &fTop);
	Model(vector<vector<vector<unsigned>>> &convInput, vector<vector<unsigned>> &poolSize, vector<unsigned> &fTop);
	//Model(string &modelFile);
	vector<double> feedForward(vector<vector<vector<double>>> &inputVals);
	vector<double> feedForward(vector<vector<double>> &inputVals);
	void backProp(vector<double> &targetVals);
	void getResults(vector<double> &resultVals);
	double getError(vector<double> &targetVals);
	//void saveModel(string &fileName);
};
/*Model::Model(vector<vector<unsigned>> &convInput, vector<unsigned> &poolSize, vector<unsigned> &fTop) {
	assert(poolSize.size() == 2);
	for (int c = 0; c < convInput.size(); c++) {
		assert(convInput[c].size() == 4);
	}
	unsigned ffNetInputSize = 0;
	for (int l = 0; l < convInput.size(); l++) {
		unsigned inputX = convInput[l][2];
		unsigned inputY = convInput[l][3];
		CNet.push_back(CLayer(convInput[l][0], convInput[l][1], inputX, inputY));
		CNetDim.push_back({ inputX, inputY, convInput[l][0], convInput[l][1] });
		PNet.push_back(PoolingLayer(poolSize[0], poolSize[1], inputX - convInput[l][0] + 1, inputY - convInput[l][1] + 1));
		PNetDim.push_back({ inputX - convInput[l][0] + 1, inputY - convInput[l][1] + 1, unsigned(ceil(double(inputX - convInput[l][0] + 1) / double(poolSize[0]))), unsigned(ceil(double(inputY - convInput[l][1] + 1) / double(poolSize[1]))) });
		ffNetInputSize += unsigned(ceil(double(inputX - convInput[l][0] + 1) / double(poolSize[0]))) * unsigned(ceil(double(inputY - convInput[l][1] + 1) / double(poolSize[1])));
	}
	vector<unsigned> topology;
	topology.push_back(ffNetInputSize);
	for (int i = 0; i < fTop.size(); i++) {
		topology.push_back(fTop[i]);
	}
	FCNet = FNet(topology);
}*/
Model::Model(vector<vector<vector<unsigned>>> &convInput, vector<vector<unsigned>> &poolSize, vector<unsigned> &fTop) {
	assert(poolSize.size() == convInput.size());
	for (int c = 0; c < convInput.size(); c++) {
		assert(convInput[c][0].size() == 4);
	}
	unsigned ffNetInputSize = 0;
	for (int c = 0; c < convInput.size(); c++) {
		unsigned inputX = convInput[c][0][2];
		unsigned inputY = convInput[c][0][3];
		std::cout << "input weight: " << inputX << endl;
		std::cout << "input height: " << inputY << endl;
		CNet.push_back({});
		CNetDim.push_back({});
		for (int l = 0; l < convInput[c].size(); l++) {
			CNet.back().push_back(CLayer(convInput[c][l][0], convInput[c][l][1], inputX, inputY));
			CNetDim.back().push_back({ inputX, inputY, convInput[c][l][0], convInput[c][l][1] });
			inputX -= (convInput[c][l][0] - 1);
			inputY -= (convInput[c][l][1] - 1);
		}
		PNet.push_back(PoolingLayer(poolSize[c][0], poolSize[c][1], inputX, inputY));
		PNetDim.push_back({ inputX, inputY, unsigned(ceil(double(inputX) / double(poolSize[c][0]))), unsigned(ceil(double(inputY) / double(poolSize[c][1]))) });
		ffNetInputSize += (PNetDim.back()[2] * PNetDim.back()[3]);
	}
	vector<unsigned> topology;
	topology.push_back(ffNetInputSize);
	for (int i = 0; i < fTop.size(); i++) {
		topology.push_back(fTop[i]);
	}
	FCNet = FNet(topology);
}
/*Model::Model(string &modelFile) {
	ifstream fin(modelFile);
	unsigned numConvLayers;
	fin >> numConvLayers;
	for (int c = 0; c < numConvLayers; c++) {
		unsigned inputX;
		unsigned inputY;
		unsigned filterX;
		unsigned filterY;
		unsigned poolingX;
		unsigned poolingY;
		fin >> inputX >> inputY >> filterX >> filterY >> poolingX >> poolingY;
		CNet.push_back(CLayer(filterX, filterY, inputX, inputY));
		CNetDim.push_back({ inputX, inputY, filterX, filterY });
		PNet.push_back(PoolingLayer(poolingX, poolingY, (inputX - filterX + 1), (inputY - filterY + 1)));
		PNetDim.push_back({ inputX - filterX + 1, inputY - filterY + 1, unsigned(ceil(double(inputX - filterX + 1) / double(poolingX))), unsigned(ceil(double(inputY - filterY + 1) / double(poolingY))) });
		vector<double> cWeights;
		for (int y = 0; y < filterY; y++) {
			for (int x = 0; x < filterX; x++) {
				double cW = 0.0;
				fin >> cW;
				cWeights.push_back(cW);
			}
		}
		double biasW = 0.0;
		fin >> biasW;
		cWeights.push_back(biasW);
		CNet.back().setWeights(cWeights);
	}
	FCNet = FNet(fin);
}
void Model::saveModel(string &fileName) {
	ofstream fout(fileName);
	fout << CNet.size() << endl;
	for (int c = 0; c < CNet.size(); c++) {
		unsigned inputX;
		unsigned inputY;
		unsigned filterX;
		unsigned filterY;
		unsigned poolingX;
		unsigned poolingY;
		CNet[c].getInputSize(inputX, inputY);
		CNet[c].getFilterSize(filterX, filterY);
		PNet[c].getPoolingFilter(poolingX, poolingY);
		fout << inputX << " " << inputY << " " << filterX << " " << filterY << " " << poolingX << " " << poolingY << endl;
		vector<double> w;
		CNet[c].getWeights(w);
		for (int i = 0; i < w.size(); i++) {
			fout << w[i] << " ";
		}
		fout << endl;
	}
	FCNet.storeNet(fout);
}*/
void Model::getResults(vector<double> &resultVals) {
	FCNet.getResults(resultVals);
}
double Model::getError(vector<double> &targetVals) {
	vector<double> results;
	FCNet.getResults(results);
	double m_error = 0.0;
	for (int t = 0; t < targetVals.size(); t++) {
		m_error += (targetVals[t] - results[t]) * (targetVals[t] - results[t]);
	}
	m_error /= targetVals.size();
	return m_error;
}
/*void Model::backProp(vector<double> &targetVals) {
	FCNet.backProp(targetVals);
	vector<Neuron> fLayer;
	FCNet.getLayer(fLayer);
	unsigned fPos = 0;
	for (int p = 0; p < PNet.size(); p++) {
		vector<Neuron> targLayer;
		for (int poolingY = 0; poolingY < PNetDim[p][3]; poolingY++) {
			for (int poolingX = 0; poolingX < PNetDim[p][2]; poolingX++) {
				assert(fPos < fLayer.size());
				targLayer.push_back(fLayer[fPos]);
				fPos++;
			}
		}
		vector<vector<Neuron>> pLayer;
		PNet[p].backProp(targLayer);
		PNet[p].getLayer(pLayer);
		//assert(pLayer.size() == 1);

		CNet[p].backProp(pLayer);
	}
}*/
void Model::backProp(vector<double> &targetVals) {
	FCNet.backProp(targetVals);
	vector<Neuron> fLayer;
	FCNet.getLayer(fLayer);
	unsigned fPos = 0;
	for (int p = 0; p < PNet.size(); p++) {
		vector<Neuron> targLayer;
		for (int poolingY = 0; poolingY < PNetDim[p][3]; poolingY++) {
			for (int poolingX = 0; poolingX < PNetDim[p][2]; poolingX++) {
				assert(fPos < fLayer.size());
				targLayer.push_back(fLayer[fPos]);
				fPos++;
			}
		}
		vector<vector<Neuron>> layer;
		PNet[p].backProp(targLayer);
		PNet[p].getLayer(layer);
		//assert(pLayer.size() == 1);
		CNet[p].back().backProp(layer);
		CNet[p].back().getOutputLayers(layer);
		for (int l = CNet[p].size() - 2; l >= 0; l--) {
			CNet[p][l].backProp_hiddenLayer(layer);
			CNet[p][l].getOutputLayers(layer);
		}
	}
}
/*void Model::feedForward(vector<vector<vector<double>>> &inputVals) {
	assert(inputVals.size() == CNet.size());
	vector<double> resultVals;
	for (int c = 0; c < CNet.size(); c++) {
		CNet[c].feedForward(inputVals[c]);
		vector<vector<Neuron>> c_layer;
		CNet[c].getOutputLayers(c_layer);
		for (int i = 0; i < c_layer.size(); i++) {
			for (int j = 0; j < c_layer[i].size(); j++) {
			}
		}

		PNet[c].feedForward(c_layer);
		PNet[c].getResultVals(resultVals);
	}
	FCNet.feedForward(resultVals);
}
void Model::feedForward(vector<vector<double>> &inputVals) {
	vector<double> resultVals;
	for (int c = 0; c < CNet.size(); c++) {
		CNet[c].feedForward(inputVals);
		vector<vector<Neuron>> c_layer;
		CNet[c].getOutputLayers(c_layer);
		for (int i = 0; i < c_layer.size(); i++) {
			for (int j = 0; j < c_layer[i].size(); j++) {
			}
		}

		PNet[c].feedForward(c_layer);
		PNet[c].getResultVals(resultVals);
	}
	FCNet.feedForward(resultVals);
}*/
vector<double> Model::feedForward(vector<vector<vector<double>>> &inputVals) {
	assert(inputVals.size() == CNet.size());
	vector<double> resultVals;
	for (int c = 0; c < CNet.size(); c++) {
		assert(CNetDim[c][0][1] == inputVals[c].size() && CNetDim[c][0][0] == inputVals[c][0].size());
		vector<vector<double>> input = inputVals[c];
		for (int l = 0; l < CNet[c].size(); l++) {
			CNet[c][l].feedForward(input);
			CNet[c][l].getResultVals(input);
		}
		vector<vector<Neuron>> c_layer;
		CNet[c].back().getOutputLayers(c_layer);
		PNet[c].feedForward(c_layer);
		PNet[c].getResultVals(resultVals);
	}
	FCNet.feedForward(resultVals);
	return resultVals;
}
vector<double> Model::feedForward(vector<vector<double>> &inputVals) {
	vector<double> resultVals;
	for (int c = 0; c < CNet.size(); c++) {
		assert(CNetDim[c][0][1] == inputVals.size() && CNetDim[c][0][0] == inputVals[0].size());
		vector<vector<double>> input = inputVals;
		for (int l = 0; l < CNet[c].size(); l++) {
			CNet[c][l].feedForward(input);
			CNet[c][l].getResultVals(input);
		}
		vector<vector<Neuron>> c_layer;
		CNet[c].back().getOutputLayers(c_layer);
		PNet[c].feedForward(c_layer);
		PNet[c].getResultVals(resultVals);
	}
	FCNet.feedForward(resultVals);
	return resultVals;
}